use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("8TPAjcQxNyMQF52RPPyeJuQ48zot9RGDKRZT15cQC2h4");

#[program]
pub mod b_nonymous {
    use super::*;

    pub fn send_post(ctx: Context<SendPost>, topic: String, content: String) -> ProgramResult {
        let post: &mut Account<Post> = &mut ctx.accounts.post;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();
    
        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into())
        }
    
        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into())
        }
    
        post.author = *author.key;
        post.timestamp = clock.unix_timestamp;
        post.topic = topic;
        post.content = content;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendPost<'info> {
    #[account(init, payer = author, space = Post::LEN)]
    pub post: Account<'info, Post>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct Post {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 50 * 4;
const MAX_CONTENT_LENGTH: usize = 280 * 4; 

impl Post {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH 
        + TIMESTAMP_LENGTH 
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH 
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; 
}

#[error]
pub enum ErrorCode {
    #[msg("Topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("Content should be 280 characters long maximum.")]
    ContentTooLong,
}