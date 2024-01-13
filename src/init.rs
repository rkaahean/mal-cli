use std::env;

pub fn initialize_client(client_id: String) {
    /*
       Initialize the CLI with a client ID.
    */
    env::set_var("MAL_CLI_CLIENT_ID", client_id);
}
