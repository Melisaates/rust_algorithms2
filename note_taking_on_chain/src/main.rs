//import * as borsh from '@project-serum/borsh';
#[derive(BorshDeserialize)]

struct NoteInstructionPayload{
    id:u64,
    title:String,
    body:String,
}
impl NoteInstructionPayload{

    pub fn unpack(input:&[u8]) -> Result<Self,ProgramError>{
        let(&variant,rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData);
        let payload=NoteInstructionPayload::try_from_slice(rest).unwrap();

        Ok(match variant{
            0 => Self::CreateNote{
            title:payload.title,
            body:payload.body,
            id:payload.id
        },
            1 => Self::UpdateNote{
                title:payload.title,
                body:payload.body,
                id:payload.id
            },

            2 => Self::DeleteNote{
                id:payload.id
            },
 
            _ => return Err(ProgramError::InvalidInstructionData)

        })
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8])-> ProgramResult{
        
        let instruction=NoteInstructionPayload::unpack(instruction_data);

        match instruction{
            NoteInstructionPayload::CreateNote{title,body,id}=>{

            },
            NoteInstructionPayload::UpdateNote{title,body,id}=>{},
            
            NoteInstructionPayload::DeleteNote{id}=>{}
        }





}
