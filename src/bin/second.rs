use lambda_runtime::handler_fn;
use log::info;
use metrics_api::{client_utility::get_client, responses::*};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn handler(_req: Request, _ctx: lambda_runtime::Context) -> Response {
    info!("handling a request...");
    let mut client = get_client().await;

    println!("Got client, sending db query");
    let response = client
        .find_one::<DBTextResponse>(
            "
        SELECT {body: body} AS `response` FROM data WHERE desired = $desired
        "
            .to_owned(),
            json!({
                "desired": true
            }),
        )
        .await;

    match response {
        Ok(resp) => match resp {
            Some(db_resp) => return Ok(db_resp.response),
            None => return Err(FailureResponse {
                body:
                    "The lambda encountered an error as the correct data wasn't read from the DB."
                        .to_owned(),
            }),
        },
        Err(rpcError) => {
            println!("{} RPC Error", rpcError);
            return Err(FailureResponse {
                body: format!("{} RPC Error", rpcError),
            });
        }
    };
}
