#[ion_proc::js_fn]
fn many_inputs(_cx: &Context, _args: &Arguments, #[ion(this)] _this: Object, #[ion(convert = ConversionBehavior::EnforceRange)] _integer: i8, #[ion(strict)] _boolean: bool, #[ion(convert = (), strict)] _string: String, _function: Function, _promise: Promise, #[ion(varargs)] _values: Vec<Value>) {}
