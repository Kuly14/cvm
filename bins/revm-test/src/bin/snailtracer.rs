use std::time::Duration;

use revm::{
    db::BenchmarkDB,
    interpreter::analysis::to_analysed,
    primitives::{address, bytes, Bytecode, Bytes, TransactTo},
    Evm,
};

pub fn simple_example() {
    let bytecode = to_analysed(Bytecode::new_raw(CONTRACT_DATA.clone()));

    // BenchmarkDB is dummy state that implements Database trait.
    let mut evm = Evm::builder()
        .with_db(BenchmarkDB::new_bytecode(bytecode.clone()))
        .modify_tx_env(|tx| {
            // execution globals block hash/energy_limit/coinbase/timestamp..
            tx.caller = address!("1000000000000000000000000000000000000000");
            tx.transact_to = TransactTo::Call(address!("0000000000000000000000000000000000000000"));
            tx.data = bytes!("30627b7c");
        })
        .build();

    // Microbenchmark
    let bench_options = microbench::Options::default().time(Duration::from_secs(2));

    microbench::bench(
        &bench_options,
        "Snailtracer Host+Interpreter benchmark",
        || {
            let _ = evm.transact();
        },
    );
}

fn main() {
    //println!("Running snailtracer bench!");
    simple_example();
    //println!("end!");
}

static CONTRACT_DATA : Bytes = bytes!("608060405234801561001057600080fd5b506004361061004c5760003560e01c806330627b7c1461005157806375ac892a14610085578063784f13661461011d578063c294360114610146575b600080fd5b610059610163565b604080516001600160f81b03199485168152928416602084015292168183015290519081900360600190f35b6100a86004803603604081101561009b57600080fd5b50803590602001356102d1565b6040805160208082528351818301528351919283929083019185019080838360005b838110156100e25781810151838201526020016100ca565b50505050905090810190601f16801561010f5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b6100596004803603606081101561013357600080fd5b508035906020810135906040013561055b565b6100a86004803603602081101561015c57600080fd5b5035610590565b6000806000610176610400610300610834565b60405180606001604052806001546000546207d5dc028161019357fe5b058152600060208083018290526040928301919091528251600b81905583820151600c81905593830151600d819055835160608082018652928152808401959095528484015282519081018352600654815260075491810191909152600854918101919091526102259161021c916102139161020e91612ef7565b612f64565b6207d5dc612feb565b620f424061301e565b8051600e556020810151600f55604001516010556102416142dd565b61025a816102556102006101806008613064565b613212565b90506102708161025561014561021c6008613064565b905061028481610255610258806008613064565b905061029a8161025561020a61020c6008613064565b90506102a781600461301e565b90506102b1613250565b8051602082015160409092015160f891821b9692821b9550901b92509050565b606060005b6000548112156104c95760006102ed828686613064565b90506002816000015160f81b90808054603f811680603e811461032a576002830184556001831661031c578192505b600160028404019350610342565b600084815260209081902060ff198516905560419094555b505050600190038154600116156103685790600052602060002090602091828204019190065b909190919091601f036101000a81548160ff02191690600160f81b840402179055506002816020015160f81b90808054603f811680603e81146103c557600283018455600183166103b7578192505b6001600284040193506103dd565b600084815260209081902060ff198516905560419094555b505050600190038154600116156104035790600052602060002090602091828204019190065b909190919091601f036101000a81548160ff02191690600160f81b840402179055506002816040015160f81b90808054603f811680603e81146104605760028301845560018316610452578192505b600160028404019350610478565b600084815260209081902060ff198516905560419094555b5050506001900381546001161561049e5790600052602060002090602091828204019190065b815460ff601f929092036101000a9182021916600160f81b90930402919091179055506001016102d6565b506002805460408051602060018416156101000260001901909316849004601f8101849004840282018401909252818152929183018282801561054d5780601f106105225761010080835404028352916020019161054d565b820191906000526020600020905b81548152906001019060200180831161053057829003601f168201915b505050505090505b92915050565b60008060008061056c878787613064565b8051602082015160409092015160f891821b9a92821b9950901b9650945050505050565b600154606090600019015b600081126107a35760005b6000548112156107995760006105bd828487613064565b90506002816000015160f81b90808054603f811680603e81146105fa57600283018455600183166105ec578192505b600160028404019350610612565b600084815260209081902060ff198516905560419094555b505050600190038154600116156106385790600052602060002090602091828204019190065b909190919091601f036101000a81548160ff02191690600160f81b840402179055506002816020015160f81b90808054603f811680603e81146106955760028301845560018316610687578192505b6001600284040193506106ad565b600084815260209081902060ff198516905560419094555b505050600190038154600116156106d35790600052602060002090602091828204019190065b909190919091601f036101000a81548160ff02191690600160f81b840402179055506002816040015160f81b90808054603f811680603e81146107305760028301845560018316610722578192505b600160028404019350610748565b600084815260209081902060ff198516905560419094555b5050506001900381546001161561076e5790600052602060002090602091828204019190065b815460ff601f929092036101000a9182021916600160f81b90930402919091179055506001016105a6565b506000190161059b565b506002805460408051602060018416156101000260001901909316849004601f810184900484028201840190925281815292918301828280156108275780601f106107fc57610100808354040283529160200191610827565b820191906000526020600020905b81548152906001019060200180831161080a57829003601f168201915b505050505090505b919050565b8160008190555080600181905550604051806080016040528060405180606001604052806302faf08081526020016303197500815260200163119e7f8081525081526020016108a460405180606001604052806000815260200161a673198152602001620f423f19815250612f64565b815260006020808301829052604092830182905283518051600355808201516004558301516005558381015180516006559081015160075582015160085582820151600955606092830151600a805460ff1916911515919091179055815192830190915260015490548291906207d5dc028161091c57fe5b058152600060208083018290526040928301919091528251600b81905583820151600c81905593830151600d819055835160608082018652928152808401959095528484015282519081018352600654815260075491810191909152600854918101919091526109979161021c916102139161020e91612ef7565b8051600e55602080820151600f55604091820151601055815160a08101835264174876e8008152825160608082018552641748862a40825263026e8f00828501526304dd1e008286015282840191825284518082018652600080825281860181905281870181905284870191825286518084018852620b71b081526203d09081880181905281890152928501928352608085018181526011805460018082018355919093528651600b9093027f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c688101938455955180517f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c69880155808901517f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c6a8801558901517f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c6b870155925180517f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c6c870155808801517f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c6d8701558801517f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c6e860155925180517f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c6f860155958601517f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c7085015594909501517f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c71830155517f31ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c72909101805492949192909160ff1990911690836002811115610c1057fe5b0217905550505060116040518060a0016040528064174876e8008152602001604051806060016040528064174290493f19815260200163026e8f0081526020016304dd1e008152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806203d09081526020016203d0908152602001620b71b0815250815260200160006002811115610cb657fe5b905281546001818101845560009384526020938490208351600b90930201918255838301518051838301558085015160028085019190915560409182015160038501558185015180516004860155808701516005860155820151600685015560608501518051600786015595860151600885015594015160098301556080830151600a83018054949593949193909260ff1990921691908490811115610d5857fe5b0217905550505060116040518060a0016040528064174876e800815260200160405180606001604052806302faf080815260200163026e8f00815260200164174876e800815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620b71b08152602001620b71b08152602001620b71b0815250815260200160006002811115610dfd57fe5b905281546001818101845560009384526020938490208351600b90930201918255838301518051838301558085015160028085019190915560409182015160038501558185015180516004860155808701516005860155820151600685015560608501518051600786015595860151600885015594015160098301556080830151600a83018054949593949193909260ff1990921691908490811115610e9f57fe5b0217905550505060116040518060a0016040528064174876e800815260200160405180606001604052806302faf080815260200163026e8f00815260200164173e54e97f1981525081526020016040518060600160405280600081526020016000815260200160008152508152602001604051806060016040528060008152602001600081526020016000815250815260200160006002811115610f3f57fe5b905281546001818101845560009384526020938490208351600b90930201918255838301518051838301558085015160028085019190915560409182015160038501558185015180516004860155808701516005860155820151600685015560608501518051600786015595860151600885015594015160098301556080830151600a83018054949593949193909260ff1990921691908490811115610fe157fe5b0217905550505060116040518060a0016040528064174876e800815260200160405180606001604052806302faf080815260200164174876e80081526020016304dd1e00815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620b71b08152602001620b71b08152602001620b71b081525081526020016000600281111561108657fe5b905281546001818101845560009384526020938490208351600b90930201918255838301518051838301558085015160028085019190915560409182015160038501558185015180516004860155808701516005860155820151600685015560608501518051600786015595860151600885015594015160098301556080830151600a83018054949593949193909260ff199092169190849081111561112857fe5b0217905550505060116040518060a0016040528064174876e800815260200160405180606001604052806302faf080815260200164174399c9ff1981526020016304dd1e00815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620b71b08152602001620b71b08152602001620b71b08152508152602001600060028111156111ce57fe5b905281546001818101845560009384526020938490208351600b90930201918255838301518051838301558085015160028085019190915560409182015160038501558185015180516004860155808701516005860155820151600685015560608501518051600786015595860151600885015594015160098301556080830151600a83018054949593949193909260ff199092169190849081111561127057fe5b0217905550505060116040518060a0016040528062fbc5208152602001604051806060016040528063019bfcc0815260200162fbc52081526020016302cd29c0815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e5881525081526020016001600281111561131157fe5b905281546001818101845560009384526020938490208351600b90930201918255838301518051838301558085015160028085019190915560409182015160038501558185015180516004860155808701516005860155820151600685015560608501518051600786015595860151600885015594015160098301556080830151600a83018054949593949193909260ff19909216919084908111156113b357fe5b0217905550505060116040518060a001604052806323c34600815260200160405180606001604052806302faf080815260200163289c455081526020016304dd1e008152508152602001604051806060016040528062b71b00815260200162b71b00815260200162b71b00815250815260200160405180606001604052806000815260200160008152602001600081525081526020016000600281111561145657fe5b905281546001818101845560009384526020938490208351600b90930201918255838301518051838301558085015160028085019190915560409182015160038501558185015180516004860155808701516005860155820151600685015560608501518051600786015595860151600885015594015160098301556080830151600a83018054949593949193909260ff19909216919084908111156114f857fe5b0217905550505060126040518060e00160405280604051806060016040528063035e1f208152602001630188c2e081526020016304a62f8081525081526020016040518060600160405280630459e4408152602001630188c2e081526020016305a1f4a081525081526020016040518060600160405280630459e44081526020016302f34f6081526020016304a62f808152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e5881525081526020016001600281111561160c57fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff19909216919084908111156116fd57fe5b0217905550505060126040518060e00160405280604051806060016040528063035e1f20815260200163016a8c8081526020016304a62f8081525081526020016040518060600160405280630459e4408152602001600081526020016304a62f8081525081526020016040518060600160405280630459e440815260200163016a8c8081526020016305a1f4a08152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e5881525081526020016001600281111561180e57fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff19909216919084908111156118ff57fe5b0217905550505060126040518060e001604052806040518060600160405280630555a9608152602001630188c2e081526020016304a62f8081525081526020016040518060600160405280630459e44081526020016302f34f6081526020016304a62f8081525081526020016040518060600160405280630459e4408152602001630188c2e081526020016305a1f4a08152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e58815250815260200160016002811115611a1357fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff1990921691908490811115611b0457fe5b0217905550505060126040518060e001604052806040518060600160405280630555a960815260200163016a8c8081526020016304a62f8081525081526020016040518060600160405280630459e440815260200163016a8c8081526020016305a1f4a081525081526020016040518060600160405280630459e4408152602001600081526020016304a62f808152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e58815250815260200160016002811115611c1557fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff1990921691908490811115611d0657fe5b0217905550505060126040518060e00160405280604051806060016040528063035e1f208152602001630188c2e081526020016304a62f8081525081526020016040518060600160405280630459e44081526020016302f34f6081526020016304a62f8081525081526020016040518060600160405280630459e4408152602001630188c2e081526020016303aa6a608152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e58815250815260200160016002811115611e1a57fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff1990921691908490811115611f0b57fe5b0217905550505060126040518060e00160405280604051806060016040528063035e1f20815260200163016a8c8081526020016304a62f8081525081526020016040518060600160405280630459e440815260200163016a8c8081526020016303aa6a6081525081526020016040518060600160405280630459e4408152602001600081526020016304a62f808152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e5881525081526020016001600281111561201c57fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff199092169190849081111561210d57fe5b0217905550505060126040518060e001604052806040518060600160405280630555a9608152602001630188c2e081526020016304a62f8081525081526020016040518060600160405280630459e4408152602001630188c2e081526020016303aa6a6081525081526020016040518060600160405280630459e44081526020016302f34f6081526020016304a62f808152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e5881525081526020016001600281111561222157fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff199092169190849081111561231257fe5b0217905550505060126040518060e001604052806040518060600160405280630555a960815260200163016a8c8081526020016304a62f8081525081526020016040518060600160405280630459e4408152602001600081526020016304a62f8081525081526020016040518060600160405280630459e440815260200163016a8c8081526020016303aa6a608152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e5881525081526020016001600281111561242357fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff199092169190849081111561251457fe5b0217905550505060126040518060e00160405280604051806060016040528063035e1f208152602001630188c2e081526020016304a62f8081525081526020016040518060600160405280630459e4408152602001630188c2e081526020016303aa6a6081525081526020016040518060600160405280630555a9608152602001630188c2e081526020016304a62f808152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e5881525081526020016001600281111561262857fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff199092169190849081111561271957fe5b0217905550505060126040518060e00160405280604051806060016040528063035e1f208152602001630188c2e081526020016304a62f8081525081526020016040518060600160405280630555a9608152602001630188c2e081526020016304a62f8081525081526020016040518060600160405280630459e4408152602001630188c2e081526020016305a1f4a08152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e5881525081526020016001600281111561282d57fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff199092169190849081111561291e57fe5b0217905550505060126040518060e00160405280604051806060016040528063035e1f20815260200163016a8c8081526020016304a62f8081525081526020016040518060600160405280630555a960815260200163016a8c8081526020016304a62f8081525081526020016040518060600160405280630459e440815260200163016a8c8081526020016303aa6a608152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e58815250815260200160016002811115612a3257fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff1990921691908490811115612b2357fe5b0217905550505060126040518060e00160405280604051806060016040528063035e1f20815260200163016a8c8081526020016304a62f8081525081526020016040518060600160405280630459e440815260200163016a8c8081526020016305a1f4a081525081526020016040518060600160405280630555a960815260200163016a8c8081526020016304a62f808152508152602001604051806060016040528060008152602001600081526020016000815250815260200160405180606001604052806000815260200160008152602001600081525081526020016040518060600160405280620f3e588152602001620f3e588152602001620f3e58815250815260200160016002811115612c3757fe5b905281546001818101845560009384526020938490208351805160139094029091019283558085015183830155604090810151600280850191909155858501518051600386015580870151600486015582015160058501558185015180516006860155808701516007860155820151600885015560608501518051600986015580870151600a860155820151600b85015560808501518051600c86015580870151600d860155820151600e85015560a08501518051600f860155958601516010850155940151601183015560c0830151601283018054949593949193909260ff1990921691908490811115612d2857fe5b0217905550505060005b601254811015612ef257600060128281548110612d4b57fe5b600091825260209182902060408051610140810182526013909302909101805460e08401908152600182015461010085015260028083015461012086015290845282516060818101855260038401548252600484015482880152600584015482860152858701919091528351808201855260068401548152600784015481880152600884015481860152858501528351808201855260098401548152600a84015481880152600b840154818601528186015283518082018552600c8401548152600d84015481880152600e84015481860152608086015283519081018452600f830154815260108301549581019590955260118201549285019290925260a0830193909352601283015491929160c084019160ff90911690811115612e6c57fe5b6002811115612e7757fe5b815250509050612eac61020e612e95836020015184600001516132cd565b612ea7846040015185600001516132cd565b612ef7565b60128381548110612eb957fe5b60009182526020918290208351600960139093029091019182015590820151600a820155604090910151600b9091015550600101612d32565b505050565b612eff6142dd565b604051806060016040528083602001518560400151028460400151866020015102038152602001836040015185600001510284600001518660400151020381526020018360000151856020015102846020015186600001510203815250905092915050565b612f6c6142dd565b604082015160208301518351600092612f9292918002918002919091019080020161330c565b90506040518060600160405280828560000151620f42400281612fb157fe5b058152602001828560200151620f42400281612fc957fe5b058152602001828560400151620f42400281612fe157fe5b0590529392505050565b612ff36142dd565b5060408051606081018252835183028152602080850151840290820152928101519091029082015290565b6130266142dd565b60405180606001604052808385600001518161303e57fe5b0581526020018385602001518161305157fe5b05815260200183856040015181612fe157fe5b61306c6142dd565b6000546013805463ffffffff1916918502860163ffffffff169190911790556130936142dd565b905060005b828112156131f157600061317261314c61021c613115600b60405180606001604052908160008201548152602001600182015481526020016002820154815250506207a1206000546207a1206130ec613343565b63ffffffff16816130f957fe5b0663ffffffff168d620f424002018161310e57fe5b0503612feb565b60408051606081018252600e548152600f5460208201526010549181019190915260015461025591906207a12090816130ec613343565b604080516060810182526006548152600754602082015260085491810191909152613212565b6040805160e081019091526003546080820190815260045460a083015260055460c083015291925060009181906131ae9061025586608c612feb565b81526020016131bc84612f64565b815260006020820181905260409091015290506131e5846102556131df8461336c565b8861301e565b93505050600101613098565b5061320861021c61320183613753565b60ff612feb565b90505b9392505050565b61321a6142dd565b50604080516060810182528251845101815260208084015181860151019082015291810151928101519092019181019190915290565b60008080556001819055613266906002906142fe565b60006003819055600481905560058190556006819055600781905560088190556009819055600a805460ff19169055600b819055600c819055600d819055600e819055600f81905560108190556132bf90601190614345565b6132cb60126000614366565b565b6132d56142dd565b5060408051606081018252825184510381526020808401518186015103908201528282015184830151039181019190915292915050565b80600260018201055b8181121561333d5780915060028182858161332c57fe5b05018161333557fe5b059050613315565b50919050565b6013805463ffffffff19811663ffffffff9182166341c64e6d0261303901821617918290551690565b6133746142dd565b600a826040015113156133a657604051806060016040528060008152602001600081526020016000815250905061082f565b60008060006133b48561379f565b91945092509050826133e857604051806060016040528060008152602001600081526020016000815250935050505061082f565b6133f0614387565b6133f86143c7565b6134006142dd565b6134086142dd565b600086600181111561341657fe5b1415613505576011858154811061342957fe5b60009182526020918290206040805160a081018252600b90930290910180548352815160608082018452600183015482526002808401548388015260038401548386015285870192909252835180820185526004840154815260058401548188015260068401548186015285850152835180820185526007840154815260088401549681019690965260098301549386019390935291830193909352600a830154919291608084019160ff909116908111156134e157fe5b60028111156134ec57fe5b8152505093508360600151915083604001519050613653565b6012858154811061351257fe5b600091825260209182902060408051610140810182526013909302909101805460e08401908152600182015461010085015260028083015461012086015290845282516060818101855260038401548252600484015482880152600584015482860152858701919091528351808201855260068401548152600784015481880152600884015481860152858501528351808201855260098401548152600a84015481880152600b840154818601528186015283518082018552600c8401548152600d84015481880152600e84015481860152608086015283519081018452600f830154815260108301549581019590955260118201549285019290925260a0830193909352601283015491929160c084019160ff9091169081111561363357fe5b600281111561363e57fe5b8152505092508260a001519150826080015190505b6040820151600190811215613669575060408201515b808360200151131561367c575060208201515b808360400151131561368f575060408201515b60408a01805160010190819052600512156136f75780620f42406136b1613343565b63ffffffff16816136be57fe5b0663ffffffff1612156136e8576136e16136db84620f4240612feb565b8261301e565b92506136f7565b50965061082f95505050505050565b6136ff6142dd565b600088600181111561370d57fe5b14156137255761371e8b878b613a57565b9050613733565b6137308b868b613aec565b90505b6137448361025561021c8785613baa565b9b9a5050505050505050505050565b61375b6142dd565b60405180606001604052806137738460000151613be8565b81526020016137858460200151613be8565b81526020016137978460400151613be8565b905292915050565b60008080808080805b6011548110156138c2576000613890601183815481106137c457fe5b60009182526020918290206040805160a081018252600b90930290910180548352815160608082018452600183015482526002808401548388015260038401548386015285870192909252835180820185526004840154815260058401548188015260068401548186015285850152835180820185526007840154815260088401549681019690965260098301549386019390935291830193909352600a830154919291608084019160ff9091169081111561387c57fe5b600281111561388757fe5b9052508a613c13565b90506000811380156138a957508415806138a957508481125b156138b957809450600093508192505b506001016137a8565b5060005b601254811015613a49576000613a17601283815481106138e257fe5b600091825260209182902060408051610140810182526013909302909101805460e08401908152600182015461010085015260028083015461012086015290845282516060818101855260038401548252600484015482880152600584015482860152858701919091528351808201855260068401548152600784015481880152600884015481860152858501528351808201855260098401548152600a84015481880152600b840154818601528186015283518082018552600c8401548152600d84015481880152600e84015481860152608086015283519081018452600f830154815260108301549581019590955260118201549285019290925260a0830193909352601283015491929160c084019160ff90911690811115613a0357fe5b6002811115613a0e57fe5b9052508a613cbb565b9050600081138015613a305750841580613a3057508481125b15613a4057809450600193508192505b506001016138c6565b509196909550909350915050565b613a5f6142dd565b6000613a7a856000015161025561021c886020015187612feb565b90506000613a8f61020e8387602001516132cd565b9050600085608001516002811115613aa357fe5b1415613ae1576000613ab9828860200151613e0c565b12613acd57613aca81600019612feb565b90505b613ad8868383613e31565b9250505061320b565b613ad8868383613fc1565b613af46142dd565b6000613b0f856000015161025561021c886020015187612feb565b6060860151909150620a2c2a9015613b2757506216e3605b6000620f4240613b3f87606001518960200151613e0c565b81613b4657fe5b05905060008112613b55576000035b64e8d4a5100081800281038380020281900590036000811215613b8c57613b8188858960600151613fc1565b94505050505061320b565b613b9e88858960600151868686614039565b98975050505050505050565b613bb26142dd565b50604080516060810182528251845102815260208084015181860151029082015291810151928101519092029181019190915290565b600080821215613bfa5750600061082f565b620f4240821315613c0f5750620f424061082f565b5090565b600080613c28846020015184600001516132cd565b90506000620f4240613c3e838660200151613e0c565b81613c4557fe5b865191900591506000908002613c5b8480613e0c565b838402030190506000811215613c775760009350505050610555565b613c808161330c565b90506103e88183031315613c9957900391506105559050565b6103e88183011315613caf570191506105559050565b50600095945050505050565b600080613cd0846020015185600001516132cd565b90506000613ce6856040015186600001516132cd565b90506000613cf8856020015183612ef7565b90506000620f4240613d0a8584613e0c565b81613d1157fe5b0590506103e71981138015613d2757506103e881125b15613d39576000945050505050610555565b85518751600091613d49916132cd565b9050600082613d588386613e0c565b81613d5f57fe5b0590506000811280613d735750620f424081135b15613d875760009650505050505050610555565b6000613d938388612ef7565b9050600084613da68b6020015184613e0c565b81613dad57fe5b0590506000811280613dc35750620f4240818401135b15613dd957600098505050505050505050610555565b600085613de68985613e0c565b81613ded57fe5b0590506103e88112156137445760009950505050505050505050610555565b6040808201519083015160208084015190850151845186510291020191020192915050565b613e396142dd565b6000620f424080613e48613343565b63ffffffff1681613e5557fe5b0663ffffffff16625fdfb00281613e6857fe5b0590506000620f4240613e79613343565b63ffffffff1681613e8657fe5b0663ffffffff1690506000613e9a8261330c565b6103e8029050613ea86142dd565b620186a0613eb98760000151614216565b1315613ee657604051806060016040528060008152602001620f4240815260200160008152509050613f09565b6040518060600160405280620f4240815260200160008152602001600081525090505b613f1661020e8288612ef7565b90506000613f2761020e8884612ef7565b9050613f7f61020e613f64613f5285620f424088613f448c61422e565b0281613f4c57fe5b05612feb565b61025585620f424089613f448d61424e565b6102558a613f7689620f42400361330c565b6103e802612feb565b9150613fb460405180608001604052808a81526020018481526020018b6040015181526020018b60600151151581525061336c565b9998505050505050505050565b613fc96142dd565b6000613ffb61020e8660200151613ff686620f4240613fec898c60200151613e0c565b60020281613f4c57fe5b6132cd565b90506140306040518060800160405280868152602001838152602001876040015181526020018760600151151581525061336c565b95945050505050565b6140416142dd565b60608701516000199015614053575060015b600061408961020e61021c61406c8c602001518a612feb565b613ff68b6140798a61330c565b620f42408c8e0205018802612feb565b60608a0151909150620f42408601906140ba57620f42406140aa838a613e0c565b816140b157fe5b05620f42400390505b60408a0151619c406c0c9f2c9cd04674edea40000000620ea6008480028502850285020205019060021261415e5761412a61411f60405180608001604052808d81526020018681526020018e6040015181526020018e6060015115151581525061336c565b82620f424003612feb565b92506141448361025561413e8e8e8e613fc1565b84612feb565b925061415383620f424061301e565b94505050505061420c565b600281056203d09001620f4240614173613343565b63ffffffff168161418057fe5b0663ffffffff1612156141b2576141536141a461419e8d8d8d613fc1565b83612feb565b600283056203d0900161301e565b6142056141f76141ec60405180608001604052808e81526020018781526020018f6040015181526020018f6060015115151581525061336c565b83620f424003612feb565b60028305620b71b00361301e565b9450505050505b9695505050505050565b60008082131561422757508061082f565b5060000390565b60008061423a8361424e565b905061320b81820264e8d4a510000361330c565b60005b600082121561426757625fdfb082019150614251565b5b625fdfb0821261427f57625fdfb082039150614268565b6001828160025b818313156142d457818385028161429957fe5b0585019450620f4240808788860202816142af57fe5b05816142b757fe5b600095909503940592506001810181029190910290600201614286565b50505050919050565b60405180606001604052806000815260200160008152602001600081525090565b50805460018160011615610100020316600290046000825580601f106143245750614342565b601f0160209004906000526020600020908101906143429190614401565b50565b50805460008255600b02906000526020600020908101906143429190614416565b50805460008255601302906000526020600020908101906143429190614475565b6040518060a00160405280600081526020016143a16142dd565b81526020016143ae6142dd565b81526020016143bb6142dd565b81526020016000905290565b6040518060e001604052806143da6142dd565b81526020016143e76142dd565b81526020016143f46142dd565b81526020016143a16142dd565b5b80821115613c0f5760008155600101614402565b5b80821115613c0f57600080825560018201819055600282018190556003820181905560048201819055600582018190556006820181905560078201819055600882018190556009820155600a8101805460ff19169055600b01614417565b5b80821115613c0f576000808255600182018190556002820181905560038201819055600482018190556005820181905560068201819055600782018190556008820181905560098201819055600a8201819055600b8201819055600c8201819055600d8201819055600e8201819055600f820181905560108201819055601182015560128101805460ff1916905560130161447656fea2646970667358221220037024f5647853879c58fbcc61ac3616455f6f731cc6e84f91eb5a3b4e06c00464736f6c63430007060033");
