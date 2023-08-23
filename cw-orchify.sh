
## Function to add the right features to Cargo.toml
add_feature_cargo_toml () {
  if ! grep -q "\[features\]" Cargo.toml; then
    echo "
[features]" >> Cargo.toml
    fi
    feature_name="interface"
    sed -i "/\[features\]/a $feature_name = [\"dep:cw-orch\"]" Cargo.toml
}

## The contracts are all the folder inside the `contracts directory`


## First we need to document the changes that need to be made in each folder

for d in contracts/main/*/ ; do
    echo "$d"

    ## We cd into the directory
    cd $d

    ## First we start by adding cw-orch to Cargo.toml
    cargo add cw-orch --optional --git https://github.com/AbstractSDK/cw-orchestrator --branch feature/starship

    ## Then we add a new feature to the package 
    ## This is a fix because we can't really do that via the CLI 
    ## TODO: (can we do it via cargo cli ?, chat gpt says no)
    add_feature_cargo_toml

    ## Add the interface endpoints protected with feature
    ## This will only be done if it's located in the contract.rs files
    endpoint_strings=(\
        "pub fn query("
        "pub fn instantiate("
        "pub fn execute("
        "pub fn migrate(" 
    )

    for ((i = 0; i < ${#endpoint_strings[@]}; i++))
    do
        fn=${endpoint_strings[$i]};
        echo $fn;
        sed -i 's/'"$fn"'/#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)] \/\/ cw-orch automatic \
    '"$fn"'/' src/contract.rs
    done

    ## Add the ExecuteMsgs to the msg files
    enum="pub enum ExecuteMsg";
        sed -i 's/'"$enum"'/#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))] \/\/ cw-orch automatic\
    '"$enum"'/' src/msg.rs

    ## Add the QueryMsgFns to the msg files
    enum="pub enum QueryMsg";
        sed -i 's/'"$enum"'/#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))] \/\/ cw-orch automatic\
    '"$enum"'/' src/msg.rs


    cd -
done