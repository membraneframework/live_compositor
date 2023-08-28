use std::{
    str::{from_utf8, Utf8Error},
    sync::{Arc, Mutex},
    time::Duration,
};

use bytes::{Bytes, BytesMut};

use compositor_common::{
    renderer_spec::{ImageSpec, ImageType},
    scene::Resolution,
};
use image::{codecs::gif::GifDecoder, AnimationDecoder, ImageFormat};
use resvg::{
    tiny_skia,
    usvg::{self, TreeParsing},
};

use crate::renderer::{
    texture::{NodeTexture, RGBATexture},
    RegisterCtx, RenderCtx, WgpuCtx,
};

#[derive(Clone)]
pub enum Image {
    Bitmap(Arc<BitmapAsset>),
    Animated(Arc<AnimatedAsset>),
    Svg(Arc<SvgAsset>),
}

impl Image {
    pub fn new(ctx: &RegisterCtx, spec: ImageSpec) -> Result<Self, ImageError> {
        let file = Self::download_file(&spec.url)?;
        let renderer = match spec.image_type {
            ImageType::Png => {
                let asset = BitmapAsset::new(&ctx.wgpu_ctx, file, ImageFormat::Png)?;
                Image::Bitmap(Arc::new(asset))
            }
            ImageType::Jpeg => {
                let asset = BitmapAsset::new(&ctx.wgpu_ctx, file, ImageFormat::Jpeg)?;
                Image::Bitmap(Arc::new(asset))
            }
            ImageType::Svg { resolution } => {
                let asset = SvgAsset::new(&ctx.wgpu_ctx, file, resolution)?;
                Image::Svg(Arc::new(asset))
            }
            ImageType::Gif => {
                let asset = AnimatedAsset::new(&ctx.wgpu_ctx, file.clone(), ImageFormat::Gif);
                match asset {
                    Ok(asset) => Image::Animated(Arc::new(asset)),
                    Err(AnimatedError::SingleFrame) => {
                        let asset = BitmapAsset::new(&ctx.wgpu_ctx, file, ImageFormat::Gif)?;
                        Image::Bitmap(Arc::new(asset))
                    }
                    Err(err) => return Err(ImageError::from(err)),
                }
            }
        };
        Ok(renderer)
    }

    fn download_file(url: &str) -> Result<bytes::Bytes, ImageError> {
        // TODO: support local files
        let response = reqwest::blocking::get(url)?;
        let response = response.error_for_status()?;
        Ok(response.bytes()?)
    }
}

pub enum ImageNode {
    Bitmap {
        asset: Arc<BitmapAsset>,
        state: Mutex<BitmapNodeState>,
    },
    Animated {
        asset: Arc<AnimatedAsset>,
        state: Mutex<AnimatedNodeState>,
    },
    Svg {
        asset: Arc<SvgAsset>,
        state: Mutex<SvgNodeState>,
    },
}

impl ImageNode {
    pub fn new(image: Image) -> Self {
        match image {
            Image::Bitmap(asset) => Self::Bitmap {
                asset,
                state: BitmapNodeState {
                    was_rendered: false,
                }
                .into(),
            },
            Image::Animated(asset) => Self::Animated {
                asset,
                state: AnimatedNodeState { first_pts: None }.into(),
            },
            Image::Svg(asset) => Self::Svg {
                asset,
                state: SvgNodeState {
                    was_rendered: false,
                }
                .into(),
            },
        }
    }

    pub fn render(&self, ctx: &mut RenderCtx, target: &NodeTexture, pts: Duration) {
        match self {
            ImageNode::Bitmap { asset, state } => asset.render(ctx.wgpu_ctx, target, state),
            ImageNode::Animated { asset, state } => asset.render(ctx.wgpu_ctx, target, state, pts),
            ImageNode::Svg { asset, state } => asset.render(ctx.wgpu_ctx, target, state),
        }
    }

    pub fn resolution(&self) -> Resolution {
        match self {
            ImageNode::Bitmap { asset, .. } => asset.resolution(),
            ImageNode::Animated { asset, .. } => asset.resolution(),
            ImageNode::Svg { asset, .. } => asset.resolution(),
        }
    }
}

pub struct BitmapNodeState {
    was_rendered: bool,
}

pub struct BitmapAsset {
    texture: RGBATexture,
}

impl BitmapAsset {
    fn new(ctx: &WgpuCtx, data: Bytes, format: ImageFormat) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory_with_format(&data, format)?;
        let texture = RGBATexture::new(
            ctx,
            Resolution {
                width: img.width() as usize,
                height: img.height() as usize,
            },
        );
        texture.upload(ctx, &img.to_rgba8());
        ctx.queue.submit([]);

        Ok(Self { texture })
    }

    fn render(&self, ctx: &WgpuCtx, target: &NodeTexture, state: &Mutex<BitmapNodeState>) {
        let mut state = state.lock().unwrap();
        if state.was_rendered {
            return;
        }

        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("copy static image asset to texture"),
            });

        let size = self.texture.size();
        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                texture: &self.texture.texture().texture,
            },
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                texture: &target.rgba_texture().texture().texture,
            },
            size,
        );

        ctx.queue.submit(Some(encoder.finish()));
        state.was_rendered = true;
    }

    fn resolution(&self) -> Resolution {
        let size = self.texture.size();
        Resolution {
            width: size.width as usize,
            height: size.height as usize,
        }
    }
}

pub struct SvgNodeState {
    was_rendered: bool,
}

pub struct SvgAsset {
    texture: RGBATexture,
}

impl SvgAsset {
    fn new(
        ctx: &WgpuCtx,
        data: Bytes,
        maybe_resolution: Option<Resolution>,
    ) -> Result<Self, SvgError> {
        let text_svg = from_utf8(&data)?;
        let tree = usvg::Tree::from_str(text_svg, &Default::default())?;
        let tree = resvg::Tree::from_usvg(&tree);
        let resolution = maybe_resolution.unwrap_or_else(|| Resolution {
            width: tree.size.width() as usize,
            height: tree.size.height() as usize,
        });

        let mut buffer = BytesMut::zeroed(resolution.width * resolution.height * 4);
        let mut pixmap = tiny_skia::PixmapMut::from_bytes(
            &mut buffer,
            resolution.width as u32,
            resolution.height as u32,
        )
        .unwrap();

        let transform = match maybe_resolution {
            Some(_) => {
                let scale_multiplier = f32::min(
                    resolution.width as f32 / tree.size.width(),
                    resolution.height as f32 / tree.size.height(),
                );
                tiny_skia::Transform::from_scale(scale_multiplier, scale_multiplier)
            }
            None => tiny_skia::Transform::default(),
        };

        tree.render(transform, &mut pixmap);

        let texture = RGBATexture::new(ctx, resolution);
        texture.upload(ctx, pixmap.data_mut());
        ctx.queue.submit([]);

        Ok(Self { texture })
    }

    fn render(&self, ctx: &WgpuCtx, target: &NodeTexture, state: &Mutex<SvgNodeState>) {
        let mut state = state.lock().unwrap();
        if state.was_rendered {
            return;
        }

        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("copy static image asset to texture"),
            });

        let size = self.texture.size();
        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                texture: &self.texture.texture().texture,
            },
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                texture: &target.rgba_texture().texture().texture,
            },
            size,
        );

        ctx.queue.submit(Some(encoder.finish()));
        state.was_rendered = true;
    }

    fn resolution(&self) -> Resolution {
        let size = self.texture.size();
        Resolution {
            width: size.width as usize,
            height: size.height as usize,
        }
    }
}

pub struct AnimatedNodeState {
    first_pts: Option<Duration>,
}

pub struct AnimatedAsset {
    frames: Vec<AnimationFrame>,
    animation_duration: Duration,
}

struct AnimationFrame {
    texture: RGBATexture,
    pts: Duration,
}

impl AnimatedAsset {
    fn new(ctx: &WgpuCtx, data: Bytes, format: ImageFormat) -> Result<Self, AnimatedError> {
        let decoded_frames = match format {
            ImageFormat::Gif => GifDecoder::new(&data[..])?.into_frames(),
            _ => todo!(),
        };

        let mut animation_duration: Duration = Duration::ZERO;
        let mut frames = vec![];
        for frame in decoded_frames {
            let frame = &frame?;
            let buffer = frame.buffer();
            let texture = RGBATexture::new(
                ctx,
                Resolution {
                    width: buffer.width() as usize,
                    height: buffer.height() as usize,
                },
            );
            texture.upload(ctx, buffer);

            let delay: Duration = frame.delay().into();
            animation_duration += delay;
            frames.push(AnimationFrame {
                texture,
                pts: animation_duration,
            });

            if frames.len() > 1000 {
                return Err(AnimatedError::TooMuchFrames);
            }
        }

        let Some(first_frame) = frames.first() else {
            return Err(AnimatedError::NoFrames);
        };
        if frames.len() == 1 {
            return Err(AnimatedError::SingleFrame);
        }
        let first_frame_size = first_frame.texture.size();
        if !frames
            .iter()
            .all(|frame| frame.texture.size() == first_frame_size)
        {
            return Err(AnimatedError::UnsupportedVariableResolution);
        }

        ctx.queue.submit([]);

        // In case only one frame, where first delay is zero
        if animation_duration.is_zero() {
            animation_duration = Duration::from_nanos(1)
        }

        Ok(Self {
            frames,
            animation_duration,
        })
    }

    fn render(
        &self,
        ctx: &WgpuCtx,
        target: &NodeTexture,
        state: &Mutex<AnimatedNodeState>,
        pts: Duration,
    ) {
        let mut state = state.lock().unwrap();
        let first_pts = match state.first_pts {
            Some(first_pts) => first_pts,
            None => {
                state.first_pts = Some(pts);
                pts
            }
        };

        let animation_pts = Duration::from_nanos(
            ((pts.as_nanos() - first_pts.as_nanos()) % self.animation_duration.as_nanos()) as u64,
        );

        let closest_frame = self
            .frames
            .iter()
            .min_by_key(|frame| u128::abs_diff(frame.pts.as_nanos(), animation_pts.as_nanos()))
            .unwrap();
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("copy static image asset to texture"),
            });

        let size = closest_frame.texture.size();
        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                texture: &closest_frame.texture.texture().texture,
            },
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                texture: &target.rgba_texture().texture().texture,
            },
            size,
        );

        ctx.queue.submit(Some(encoder.finish()));
    }

    fn resolution(&self) -> Resolution {
        let size = self.frames.first().unwrap().texture.size();
        Resolution {
            width: size.width as usize,
            height: size.height as usize,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    #[error("Failed to download asset")]
    AssetDownload(#[from] reqwest::Error),

    #[error("Failed to read as bitmap")]
    FailedToReadAsBitmap(#[from] image::ImageError),

    #[error("Failed to read SVG")]
    ParsingSvgFailed(#[from] SvgError),

    #[error("Failed to read image with animations")]
    ParsingAnimatedFailed(#[from] AnimatedError),
}

#[derive(Debug, thiserror::Error)]
pub enum SvgError {
    #[error("Invalid utf-8 content")]
    InvalidUtf8Content(#[from] Utf8Error),

    #[error("Failed to parse the svg image")]
    ParsingSvgFailed(#[from] usvg::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AnimatedError {
    #[error("To much frames")]
    TooMuchFrames,

    #[error("Single frame")]
    SingleFrame,

    #[error("No frames")]
    NoFrames,

    #[error("Detected variable resolution")]
    UnsupportedVariableResolution,

    #[error("Failed to parse image")]
    FailedToParse(#[from] image::ImageError),
}
