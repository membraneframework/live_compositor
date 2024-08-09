# Changelog

## unreleased

### 💥 Breaking changes

- Remove `forward_error_correction` option from RTP OPUS output. ([#615](https://github.com/software-mansion/live-compositor/pull/615) by [@wkozyra95](https://github.com/wkozyra95))

### ✨ New features

- Support DeckLink cards as an input. ([#587](https://github.com/software-mansion/live-compositor/pull/587), [#597](https://github.com/software-mansion/live-compositor/pull/597), [#598](https://github.com/software-mansion/live-compositor/pull/598), [#599](https://github.com/software-mansion/live-compositor/pull/599) by [@wkozyra95](https://github.com/wkozyra95))
- Add `LIVE_COMPOSITOR_INPUT_BUFFER_DURATION_MS` environment variable to control input stream buffer size. ([#600](https://github.com/software-mansion/live-compositor/pull/600) by [@wkozyra95](https://github.com/wkozyra95))
- Add endpoint for requesting keyframe on the output stream. ([#620](https://github.com/software-mansion/live-compositor/pull/620) by [@WojciechBarczynski](https://github.com/WojciechBarczynski))
- Add MP4 output ([#657](https://github.com/software-mansion/live-compositor/pull/657) by [@WojciechBarczynski](https://github.com/WojciechBarczynski))

### 🐛 Bug fixes

- Fix input queueing when some of the inputs do not produce frames/samples . ([#625](https://github.com/software-mansion/live-compositor/pull/625) by [@wkozyra95](https://github.com/wkozyra95))

## [v0.2.0](https://github.com/software-mansion/live-compositor/releases/tag/v0.2.0)

Initial release
