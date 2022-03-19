import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BNonymous } from "../target/types/b_nonymous";
import * as assert from "assert";
import * as bs58 from "bs58";

describe("b-nonymous", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.BNonymous as Program<BNonymous>;

  it('can post', async () => {
    const post = anchor.web3.Keypair.generate();
    await program.rpc.SendPost('banana', 'Pepe', {
        accounts: {
            post: post.publicKey,
            author: program.provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [post],
    });

 
    const postAccount = await program.account.post.fetch(post.publicKey);

    assert.equal(postAccount.author.toBase58(), program.provider.wallet.publicKey.toBase58());
    assert.equal(postAccount.topic, 'banana');
    assert.equal(postAccount.content, 'Pepe');
    assert.ok(postAccount.timestamp);
});
});
