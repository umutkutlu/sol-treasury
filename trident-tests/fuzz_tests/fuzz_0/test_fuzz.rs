use sol_treasury::entry as entry_sol_treasury;
use sol_treasury::ID as PROGRAM_ID_SOL_TREASURY;
const PROGRAM_NAME_SOL_TREASURY: &str = "sol_treasury";
use fuzz_instructions::sol_treasury_fuzz_instructions::FuzzInstruction as FuzzInstruction_sol_treasury;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_sol_treasury;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(
                PROGRAM_NAME_SOL_TREASURY,
                &PROGRAM_ID_SOL_TREASURY,
                processor!(convert_entry!(entry_sol_treasury)));

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_SOL_TREASURY, &mut client);
        });
    }
}
