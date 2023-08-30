// automatically generate tests that can be used for extrinsic testing
use crate::error::Error;
use crate::jsonrpseeclient::JsonrpseeClient;
use crate::test_helper::InputHelper;

pub async fn generate_test(client: JsonrpseeClient) -> Result<(), Error> {
    /// raw type to match against the input gen
    Ok(())
}
