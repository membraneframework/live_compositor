import * as Api from '../api';
import LiveCompositorComponent, { SceneBuilder, SceneComponent } from '../component';

type ImageProps = {
  children?: undefined;

  /**
   * Id of a component.
   */
  id?: Api.ComponentId | null;
  /**
   * Id of an image. It identifies an image registered using a [`register image`](../routes.md#register-image) request.
   */
  imageId: Api.RendererId;
};

class Image extends LiveCompositorComponent<ImageProps> {
  builder: SceneBuilder<ImageProps> = sceneBuilder;
}

function sceneBuilder(props: ImageProps, _children: SceneComponent[]): Api.Component {
  return {
    type: 'image',
    id: props.id,
    image_id: props.imageId,
  };
}

export default Image;
