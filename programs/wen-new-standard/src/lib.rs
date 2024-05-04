// Importa los tipos necesarios al ámbito actual
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use crate::my_module::MyType;
use crate::instructions::token::mint::handler; // Suponiendo que handler está en el archivo instructions/token/mint.rs

// Declara los módulos y hace uso de las macros necesarias
pub mod instructions;
pub use instructions::*;

// Declara el ID
declare_id!("Gj2DAzAnkm1zHTManYvGsgaBYzbsyBboKRaAKYEHZuwc");

// Define el módulo del programa
#[program]
pub mod wen_new_standard {
    // Importa los símbolos necesarios al ámbito actual
    use super::*;
    use crate::token::MintArgs;
    use crate::token::MintNft;

    // Define la función mint
    pub fn mint(
        ctx: Context<MintNft>,
        args: MintArgs,
    ) -> ProgramResult {
        // Llama al handler de mint en el módulo de instrucciones
        handler(ctx, args)
    }
}
