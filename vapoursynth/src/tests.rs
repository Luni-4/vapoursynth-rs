#![cfg(test)]
use super::*;

// We need the VSScript functions, and either VSScript API 3.2 or the VapourSynth functions.
#[cfg(all(feature = "vsscript-functions",
          any(feature = "vapoursynth-functions", feature = "gte-vsscript-api-32")))]
mod need_api {
    use super::*;
    use video_info::{Framerate, Resolution};

    #[test]
    fn green() {
        let api = API::get().unwrap();
        let env = vsscript::Environment::from_file("test-vpy/green.vpy", vsscript::EvalFlags::Nothing)
            .unwrap();
        let node = env.get_output(api, 0).unwrap();
        let info = node.info();

        if let Property::Constant(format) = info.format {
            assert_eq!(format.name().to_string_lossy(), "RGB24");
        } else {
            assert!(false);
        }

        assert_eq!(
            info.framerate,
            Property::Constant(Framerate {
                numerator: 60,
                denominator: 1,
            })
        );
        assert_eq!(
            info.resolution,
            Property::Constant(Resolution {
                width: 1920,
                height: 1080,
            })
        );

        #[cfg(feature = "gte-vapoursynth-api-32")]
        assert_eq!(info.num_frames, 100);
        #[cfg(not(feature = "gte-vapoursynth-api-32"))]
        assert_eq!(info.num_frames, Property::Constant(100));

        let frame = node.get_frame(0).unwrap();
        let format = frame.format();
        assert_eq!(format.name().to_string_lossy(), "RGB24");
        assert_eq!(format.plane_count(), 3);

        for plane in 0..format.plane_count() {
            let resolution = frame.resolution(plane);
            assert_eq!(
                resolution,
                Resolution {
                    width: 1920,
                    height: 1080,
                }
            );

            let color = if plane == 1 { [255; 1920] } else { [0; 1920] };

            let stride = frame.stride(plane);
            let plane = frame.data(plane);

            for row in 0..resolution.height {
                assert_eq!(
                    &plane[row * stride..row * stride + resolution.width],
                    &color[..]
                );
            }
        }

        let props = frame.props();
        assert_eq!(props.key_count(), 2);
        assert_eq!(props.key(0).to_string_lossy(), "_DurationDen");
        assert_eq!(props.key(1).to_string_lossy(), "_DurationNum");

        assert_eq!(props.value_count(props.key(0)), Some(1));
        if let Ok(Value::Int(60)) = props.value(props.key(0), 0) {
        } else {
            assert!(false);
        }
        assert_eq!(props.value_count(props.key(1)), Some(1));
        if let Ok(Value::Int(1)) = props.value(props.key(1), 0) {
        } else {
            assert!(false);
        }

        let mut map = Map::new(api);
        assert!(env.get_variable("video", &mut map.get_ref_mut()).is_ok());
        let value = map.iter().next();
        assert!(value.is_some());
        let value = value.unwrap();
        assert_eq!(value.0.to_string_lossy(), "video");
        if let ValueArray::Nodes(x) = value.1 {
            assert_eq!(x.len(), 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn green_from_string() {
        let api = API::get().unwrap();
        let env = vsscript::Environment::from_script(include_str!("../test-vpy/green.vpy")).unwrap();
        let node = env.get_output(api, 0).unwrap();
        let info = node.info();

        if let Property::Constant(format) = info.format {
            assert_eq!(format.name().to_string_lossy(), "RGB24");
        } else {
            assert!(false);
        }

        assert_eq!(
            info.framerate,
            Property::Constant(Framerate {
                numerator: 60,
                denominator: 1,
            })
        );
        assert_eq!(
            info.resolution,
            Property::Constant(Resolution {
                width: 1920,
                height: 1080,
            })
        );

        #[cfg(feature = "gte-vapoursynth-api-32")]
        assert_eq!(info.num_frames, 100);
        #[cfg(not(feature = "gte-vapoursynth-api-32"))]
        assert_eq!(info.num_frames, Property::Constant(100));

        let frame = node.get_frame(0).unwrap();
        let format = frame.format();
        assert_eq!(format.name().to_string_lossy(), "RGB24");
        assert_eq!(format.plane_count(), 3);

        for plane in 0..format.plane_count() {
            let resolution = frame.resolution(plane);
            assert_eq!(
                resolution,
                Resolution {
                    width: 1920,
                    height: 1080,
                }
            );

            let color = if plane == 1 { [255; 1920] } else { [0; 1920] };

            let stride = frame.stride(plane);
            let plane = frame.data(plane);

            for row in 0..resolution.height {
                assert_eq!(
                    &plane[row * stride..row * stride + resolution.width],
                    &color[..]
                );
            }
        }

        let props = frame.props();
        assert_eq!(props.key_count(), 2);
        assert_eq!(props.key(0).to_string_lossy(), "_DurationDen");
        assert_eq!(props.key(1).to_string_lossy(), "_DurationNum");

        assert_eq!(props.value_count(props.key(0)), Some(1));
        if let Ok(Value::Int(60)) = props.value(props.key(0), 0) {
        } else {
            assert!(false);
        }
        assert_eq!(props.value_count(props.key(1)), Some(1));
        if let Ok(Value::Int(1)) = props.value(props.key(1), 0) {
        } else {
            assert!(false);
        }

        let mut map = Map::new(api);
        assert!(env.get_variable("video", &mut map.get_ref_mut()).is_ok());
        let value = map.iter().next();
        assert!(value.is_some());
        let value = value.unwrap();
        assert_eq!(value.0.to_string_lossy(), "video");
        if let ValueArray::Nodes(x) = value.1 {
            assert_eq!(x.len(), 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn variable() {
        let api = API::get().unwrap();
        let env =
            vsscript::Environment::from_file("test-vpy/variable.vpy", vsscript::EvalFlags::Nothing)
                .unwrap();
        let node = env.get_output(api, 0).unwrap();
        let info = node.info();

        assert_eq!(info.format, Property::Variable);
        assert_eq!(info.framerate, Property::Variable);
        assert_eq!(info.resolution, Property::Variable);

        #[cfg(feature = "gte-vapoursynth-api-32")]
        assert_eq!(info.num_frames, 200);
        #[cfg(not(feature = "gte-vapoursynth-api-32"))]
        assert_eq!(info.num_frames, Property::Constant(200));

        // Test the first frame.
        let frame = node.get_frame(0).unwrap();
        let format = frame.format();
        assert_eq!(format.name().to_string_lossy(), "RGB24");
        assert_eq!(format.plane_count(), 3);

        for plane in 0..format.plane_count() {
            let resolution = frame.resolution(plane);
            assert_eq!(
                resolution,
                Resolution {
                    width: 1920,
                    height: 1080,
                }
            );

            let color = if plane == 1 { [255; 1920] } else { [0; 1920] };

            let stride = frame.stride(plane);
            let plane = frame.data(plane);

            for row in 0..resolution.height {
                assert_eq!(&plane[row * stride..(row + 1) * stride], &color[..]);
            }
        }

        let props = frame.props();
        assert_eq!(props.key_count(), 2);
        assert_eq!(props.key(0).to_string_lossy(), "_DurationDen");
        assert_eq!(props.key(1).to_string_lossy(), "_DurationNum");

        assert_eq!(props.value_count(props.key(0)), Some(1));
        if let Ok(Value::Int(60)) = props.value(props.key(0), 0) {
        } else {
            assert!(false);
        }
        assert_eq!(props.value_count(props.key(1)), Some(1));
        if let Ok(Value::Int(1)) = props.value(props.key(1), 0) {
        } else {
            assert!(false);
        }

        // Test the first frame of the next format.
        let frame = node.get_frame(100).unwrap();
        let format = frame.format();
        assert_eq!(format.name().to_string_lossy(), "Gray8");
        assert_eq!(format.plane_count(), 1);

        let plane = 0;
        let resolution = frame.resolution(plane);
        assert_eq!(
            resolution,
            Resolution {
                width: 1280,
                height: 720,
            }
        );

        let color = [127; 1280];

        let stride = frame.stride(plane);
        let plane = frame.data(plane);

        for row in 0..resolution.height {
            assert_eq!(
                &plane[row * stride..row * stride + resolution.width],
                &color[..]
            );
        }

        let props = frame.props();
        assert_eq!(props.key_count(), 2);
        assert_eq!(props.key(0).to_string_lossy(), "_DurationDen");
        assert_eq!(props.key(1).to_string_lossy(), "_DurationNum");

        assert_eq!(props.value_count(props.key(0)), Some(1));
        if let Ok(Value::Int(30)) = props.value(props.key(0), 0) {
        } else {
            assert!(false);
        }
        assert_eq!(props.value_count(props.key(1)), Some(1));
        if let Ok(Value::Int(1)) = props.value(props.key(1), 0) {
        } else {
            assert!(false);
        }

        let mut map = Map::new(api);
        assert!(env.get_variable("video", &mut map.get_ref_mut()).is_ok());
        let value = map.iter().next();
        assert!(value.is_some());
        let value = value.unwrap();
        assert_eq!(value.0.to_string_lossy(), "video");
        if let ValueArray::Nodes(x) = value.1 {
            assert_eq!(x.len(), 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn clear_output() {
        let env = vsscript::Environment::from_script(include_str!("../test-vpy/green.vpy")).unwrap();
        assert!(env.clear_output(1).is_none());
        assert!(env.clear_output(0).is_some());
        assert!(env.clear_output(0).is_none());
    }

    #[test]
    fn iterators() {
        let api = API::get().unwrap();
        let env = vsscript::Environment::from_script(include_str!("../test-vpy/green.vpy")).unwrap();
        let node = env.get_output(api, 0).unwrap();
        let frame = node.get_frame(0).unwrap();
        let props = frame.props();

        assert_eq!(props.keys().size_hint(), (2, Some(2)));
        assert_eq!(props.iter().size_hint(), (2, Some(2)));
    }

    #[test]
    fn vsscript_variables() {
        let api = API::get().unwrap();
        let env = vsscript::Environment::from_script(include_str!("../test-vpy/green.vpy")).unwrap();

        let mut map = Map::new(api);
        assert!(env.get_variable("video", &mut map.get_ref_mut()).is_ok());
        assert!(env.clear_variable("video").is_ok());
        assert!(env.clear_variable("video").is_err());
        assert!(env.get_variable("video", &mut map.get_ref_mut()).is_err());

        assert!(env.set_variables(&map.get_ref()).is_ok());
        assert!(env.get_variable("video", &mut map.get_ref_mut()).is_ok());
    }
}
