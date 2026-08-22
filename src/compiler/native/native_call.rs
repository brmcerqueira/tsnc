#[macro_export]
macro_rules! native_call {
    (
        $(
            $object:literal: {
                $(
                    $function:literal: $handler:ident
                ),* $(,)?
            }
        ),* $(,)?
    ) => {
        pub fn native_call_resolver<'c>(
            visitor: &mut MLIRCodegenVisitor<'c>,
            args: &Vec<Value<'c, 'c>>,
            object_name: &str,
            function_name: &str,
        ) -> Result<Value<'c, 'c>> {
            $(
                $(
                    if object_name == $object && function_name == $function {
                        return $handler(visitor, args);
                    }
                )*
            )*

            Err(anyhow!(
                "can't resolve native call for {}.{}",
                object_name,
                function_name
            ))
        }
    };
}
