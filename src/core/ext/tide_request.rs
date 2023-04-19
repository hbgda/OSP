use crate::middleware::MiddlewareData;

pub trait TideRequestExt {
    fn make_data(&self, data: serde_json::Value) -> serde_json::Value;
}

impl<State: Clone + Send + Sync + 'static> TideRequestExt for tide::Request<State> {
    fn make_data(&self, mut data: serde_json::Value) -> serde_json::Value {
        let ext = match self.ext::<MiddlewareData>() {
            Some(ext) => ext.clone(),
            None => MiddlewareData::new()
        };

        data.as_object_mut().unwrap().extend(ext.as_object().unwrap().clone());
        data
    }
}