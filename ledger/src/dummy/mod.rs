use std::rc::Rc;

use binprot::BinProtRead;
use mina_p2p_messages::v2::TransactionSnarkProofStableV2;

#[cfg(test)]
use crate::VerificationKey;

/// Value of `Proof.transaction_dummy` when we run `dune runtest src/lib/staged_ledger -f`
/// The file was generated this way:
///
/// let dummy = Proof.transaction_dummy in
///
/// let buf = Bigstring.create (Proof.Stable.V2.bin_size_t dummy) in
/// ignore (Proof.Stable.V2.bin_write_t buf ~pos:0 dummy : int) ;
/// let bytes = Bigstring.to_bytes buf in
///
/// let explode s = List.init (String.length s) ~f:(fun i -> String.get s i) in
///
/// let s = (String.concat ~sep:"," (List.map (explode (Bytes.to_string bytes)) ~f:(fun b -> string_of_int (Char.to_int b)))) in
///
/// Core.Printf.eprintf !"dummy proof= %{sexp: Proof.t}\n%!" dummy;
/// Core.Printf.eprintf !"dummy proof= %s\n%!" s;
pub fn dummy_transaction_proof() -> Rc<TransactionSnarkProofStableV2> {
    let mut cursor = std::io::Cursor::new(include_bytes!("dummy_transaction_proof.bin"));
    Rc::new(TransactionSnarkProofStableV2::binprot_read(&mut cursor).unwrap())
}

// dummy_transaction_proof
//
// ((statement
//   ((proof_state
//     ((deferred_values
//       ((plonk
//         ((alpha ((inner (0a0b826d2ea44ee9 926c53269b9b0760))))
//          (beta (7f90a19136cf8a1f bb1f65ee453716e2))
//          (gamma (91f42b519c1e0c85 17dbf530a0e8ae6c))
//          (zeta ((inner (0de4ff4454670090 9ae97b3e176bd584))))
//          (joint_combiner ())))
//        (combined_inner_product
//         (Shifted_value
//          0x3EFD5AA1F6C0E3E9B0060FB78C8E094426F711FBE872DF1FB1826EEE98A8E18B))
//        (b
//         (Shifted_value
//          0x35156FEEB6695581B864A90EC9673E1D525D625F0BE01FEBE4FDDEE39722F4F5))
//        (xi ((inner (27183fa7096bbec0 fd578e6ff082620c))))
//        (bulletproof_challenges
//         (((prechallenge ((inner (ad7370b456a72ab9 364ff923d5f19efc)))))
//          ((prechallenge ((inner (59871c628e2f7c00 9cc7f673a226cc87)))))
//          ((prechallenge ((inner (2354821d9eb6f2af d5b96bd1f67df57e)))))
//          ((prechallenge ((inner (147bf7d0a09086f6 625e62ce40242a68)))))
//          ((prechallenge ((inner (afc61b633256ad0e 09807210a11fe1fb)))))
//          ((prechallenge ((inner (f9581ebecaac4191 571ed5993eb7c9a6)))))
//          ((prechallenge ((inner (aa4e50a3cd64c3bd d814e70ac1fec568)))))
//          ((prechallenge ((inner (b21dacdf825ede6d fe08a217c5db07a5)))))
//          ((prechallenge ((inner (824e0fd6e9e6aa7f fcf63a984eb97f2c)))))
//          ((prechallenge ((inner (c331882b711b04ca 9139acc7b6ae2629)))))
//          ((prechallenge ((inner (dc0f8f47fd8151ef 00a97a43c43b9587)))))
//          ((prechallenge ((inner (6038d81ed18632bf f9b3bc70a2aa05ab)))))
//          ((prechallenge ((inner (9475d2e0e5af475a d1cba702aec3d2f6)))))
//          ((prechallenge ((inner (be780ff6f92d7c04 e13d589132fbe254)))))
//          ((prechallenge ((inner (c670b712b8317513 1675cc339a483e08)))))
//          ((prechallenge ((inner (48c1b0a2b1cab8d1 1b6604e3c071b1ce)))))))
//        (branch_data ((proofs_verified N0) (domain_log2 "\014")))))
//      (sponge_digest_before_evaluations
//       (0000000000000000 0000000000000000 0000000000000000 0000000000000000))
//      (messages_for_next_wrap_proof
//       ((challenge_polynomial_commitment
//         (0x0FD332BC7DFD613056419FDB0559B4534048252D441179C1E79BF01549549962
//          0x160504505EA5EB041D033936E7B1A6553A163C7CE2E6B1C74FA2C1F78552D7AC))
//        (old_bulletproof_challenges
//         ((((prechallenge ((inner (3382b3c9ace6bf6f 79974358f9761863)))))
//           ((prechallenge ((inner (dd3a2b06e9888797 dd7ae6402944a1c7)))))
//           ((prechallenge ((inner (c6e8e530f49c9fcb 07ddbb65cda09cdd)))))
//           ((prechallenge ((inner (532c59a287691a13 a921bcb02a656f7b)))))
//           ((prechallenge ((inner (e29c77b18f10078b f85c5f00df6b0cee)))))
//           ((prechallenge ((inner (1dbda72d07b09c87 4d1b97e2e95f26a0)))))
//           ((prechallenge ((inner (9c75747c56805f11 a1fe6369facef1e8)))))
//           ((prechallenge ((inner (5c2b8adfdbe9604d 5a8c718cf210f79b)))))
//           ((prechallenge ((inner (22c0b35c51e06b48 a6888b7340a96ded)))))
//           ((prechallenge ((inner (9007d7b55e76646e c1c68b39db4e8e12)))))
//           ((prechallenge ((inner (4445e35e373f2bc9 9d40c715fc8ccde5)))))
//           ((prechallenge ((inner (429882844bbcaa4e 97a927d7d0afb7bc)))))
//           ((prechallenge ((inner (99ca3d5bfffd6e77 efe66a55155c4294)))))
//           ((prechallenge ((inner (4b7db27121979954 951fa2e06193c840)))))
//           ((prechallenge ((inner (2cd1ccbeb20747b3 5bd1de3cf264021d))))))
//          (((prechallenge ((inner (3382b3c9ace6bf6f 79974358f9761863)))))
//           ((prechallenge ((inner (dd3a2b06e9888797 dd7ae6402944a1c7)))))
//           ((prechallenge ((inner (c6e8e530f49c9fcb 07ddbb65cda09cdd)))))
//           ((prechallenge ((inner (532c59a287691a13 a921bcb02a656f7b)))))
//           ((prechallenge ((inner (e29c77b18f10078b f85c5f00df6b0cee)))))
//           ((prechallenge ((inner (1dbda72d07b09c87 4d1b97e2e95f26a0)))))
//           ((prechallenge ((inner (9c75747c56805f11 a1fe6369facef1e8)))))
//           ((prechallenge ((inner (5c2b8adfdbe9604d 5a8c718cf210f79b)))))
//           ((prechallenge ((inner (22c0b35c51e06b48 a6888b7340a96ded)))))
//           ((prechallenge ((inner (9007d7b55e76646e c1c68b39db4e8e12)))))
//           ((prechallenge ((inner (4445e35e373f2bc9 9d40c715fc8ccde5)))))
//           ((prechallenge ((inner (429882844bbcaa4e 97a927d7d0afb7bc)))))
//           ((prechallenge ((inner (99ca3d5bfffd6e77 efe66a55155c4294)))))
//           ((prechallenge ((inner (4b7db27121979954 951fa2e06193c840)))))
//           ((prechallenge ((inner (2cd1ccbeb20747b3 5bd1de3cf264021d))))))))))))
//    (messages_for_next_step_proof
//     ((app_state ()) (challenge_polynomial_commitments ())
//      (old_bulletproof_challenges ())))))
//  (prev_evals
//   ((evals
//     ((public_input
//       (0x04577E44727EC17CA729F136387A179FD37B93C2181487584C5CF6CC8291F337
//        0x10951415F96E018A54A1933B8432864D3C12D65D82879E8017B41D6643991617))
//      (evals
//       ((w
//         (((0x3DC3DF907CF74AD5F1F3EFC3C86E625C8014EB6DD963F405BA4A239CF224D204)
//           (0x3CE102EE511653BE51F3612C8F1307F1D475C6064CDDA7ACC0B7EF9214CD4229))
//          ((0x3DB918E694A9484461DEC5C06011026C88C4FF0C68DD4A10FF4FDD9008490C1F)
//           (0x2020D99993A57E6167129C513ED3ED3DD85EA72DBAF59158DCC05380C619C934))
//          ((0x2C5DA7577043B52B1F6FCE5DB412CB7E63285412F404D256448726A2B5EAEAE1)
//           (0x3C2C6457D8F63117207A23469522EBFB8F84D619F214B05ED55E8AA81EF41839))
//          ((0x13F2892FCFF1AA8228997EAE17A7A8450C85A0BD36E4831C6CD7786BF4DB8128)
//           (0x35E389211B7527C9225493812B881C6614F0ACE3DD644DC4C3456AB55B5A6485))
//          ((0x24FDB3D7D382A3C2B98E22A24E0629B20531A6B5A618A9AF0BAC83DC55EE9CAF)
//           (0x1FCB11E09517F972296C601ECFBBD414A7265C6461C855C7F94D863CEF7286F1))
//          ((0x21B3724B92C56F6BC27AE6EBE6BB4F5EB98FDCCA2755E4F74D29A8B724A8DE41)
//           (0x33A1AB344E0FDF1FCB37FFE0B39EBAEE797262E525C9ACC1F3978DC9C8912DDC))
//          ((0x3B570949A42E0EC74AD4FA98DA1A114A21A96C511EDC6E568B1FF0260F8B14DF)
//           (0x374F95A9B9B0E56DE1BC1A4BC2D93702E5CBC9598FB45C8CDD04D14732613C3A))
//          ((0x0DEE521F564D33D39F1AC6702CFC0F7EBFBA6F36B1AEBC525DF0DF32024ABAF2)
//           (0x3217BD06879B3087ADA34D6395D96FC98A040C8CC9999B834C88EA14F36D7291))
//          ((0x16B21F0A23E1A273A978695B5EF2AC3ABDDE4E0DA05BFBF8644AAAD3BC35FD44)
//           (0x22CA8F667F2004B21D2E78299FA8CE1A252EF7D88BFC30D835899EBDA0CB6236))
//          ((0x18C503E6166A9E3A7DA22AA8542FD5B01B8E866161E4F9169D7464A5BC8097B2)
//           (0x06B8F16FD29A5769B788FA6751DDBA9216C60F23D16CDB827CB0A06EC679610E))
//          ((0x3AC14783C5DAA69381FEEB8EA887E6DDE7D0F34CDEE7E2666F67B5259730EA8F)
//           (0x1CC651AF09CAB489E83C08F198A217A66EA39F1792CEC67AD44436A6365E14A6))
//          ((0x12F00C08E9F6757F29DCC4012438CD57BF6C2733D861A0572EC3E8DE2F78D9DF)
//           (0x23158354F439D8344342E71FC29D5B87FD91304AFEDFC10EAAA5C0479118A429))
//          ((0x328410040FE91571B631714D52BEDA12278381AFA1BE498E23D3BF83FF449029)
//           (0x3C170D40DE98D33B2BF50DD88C01D24BCE1D77691C33BEE7D0362D45D7256988))
//          ((0x347F341078312392F5BF115F8D18D020674359F739B43893711EDD3726F25DBB)
//           (0x28FC4B8A46CF1AC738C2A41D56AAE023125A41898380E7EB805A114FAE442998))
//          ((0x10C56080D3F326BF73405173D8E00D95050C245067C64405D0242957868C528D)
//           (0x17B1965455020E6FB48587B778566D46B294256ACD5B93300D927EA2469E2313))))
//        (z
//         ((0x20BE6DC1952B2D6F67C807E6BCBF3F0BCA8154AF8BC7C5B86DBB221207332CD2)
//          (0x2775BF922E74D84573234E2DDD7B3ECBE2514B91B6CBBF7293C785287DBD8A32)))
//        (s
//         (((0x0870FDAF0C2580691FDD8533D06A043630D3800F81F6ED496696057580EE4066)
//           (0x0FC79E2175851C844DBD6FEC443DB73B65835C4FCA0A873399A94C8231142888))
//          ((0x0EC6977EBF8046710783DEBF7E8FF7EF8780F583D8DE3BCA7E0C8D6880534A15)
//           (0x1162BB64F694523933117380F29756A013D7639DD1681B44A2B2E93099A4AA19))
//          ((0x020414F949F39E7851F11448DE78C045B2E64889D9F85871093FDC0E17F515C5)
//           (0x3ABFFC4955EB45EDD62C7D557502B072F656D00A421164FB058092FF2C40C143))
//          ((0x2DFBB12881E2C4263BFF0BD255B0CE383A9D744DFFBB15D1B5C64542420F55E1)
//           (0x31D8CE985CD9D8994C6BF7A9A486423A718343FB4DD2E9EE3C08EC462E437DAA))
//          ((0x0F98D03BF3A19159EB251B86AFED7A6D5A9B6E9E9D7515DBBFF1A2F18902B749)
//           (0x386FBEFAEE4D8E9D3352DB75AC9B731B7B88E8DAB39F5AA6FA39EDE6228CDB08))
//          ((0x2434346AEBADD24A79143D5FC222454407DC6C899A9EE59CD3FF3F37AD9811E8)
//           (0x140971802DB61C631AB419F84B3D37A0173B2DCBD0A5C66F5E35F2560B845024))))
//        (generic_selector
//         ((0x04D210116B2EE94FB68867635BB67E7B994170FE86BCD511563CB33D603688B3)
//          (0x1D7BB7B9C68312EEC2C25ED1373C0159651382516A8D1623B0A06095D3E8C02F)))
//        (poseidon_selector
//         ((0x170FEEB583692A4AC732E6DFABF031D7F50C03EAF013D4A56E485314686A62C2)
//          (0x35BCDE509A83CB4CEF5A8A39B32A5CAFC2C9F3AAD8D466234FF9ED4D30C95985)))
//        (lookup ())))))
//    (ft_eval1
//     0x38E096378D1D3B82D8A4A858685A03574EEC62F0250659FE073AAC447D6890F4)))
//  (proof
//   ((messages
//     ((w_comm
//       (((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        ((0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))))
//      (z_comm
//       ((0x0000000000000000000000000000000000000000000000000000000000000001
//         0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)))
//      (t_comm
//       ((0x0000000000000000000000000000000000000000000000000000000000000001
//         0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//        (0x0000000000000000000000000000000000000000000000000000000000000001
//         0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//        (0x0000000000000000000000000000000000000000000000000000000000000001
//         0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//        (0x0000000000000000000000000000000000000000000000000000000000000001
//         0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//        (0x0000000000000000000000000000000000000000000000000000000000000001
//         0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//        (0x0000000000000000000000000000000000000000000000000000000000000001
//         0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//        (0x0000000000000000000000000000000000000000000000000000000000000001
//         0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)))
//      (lookup ())))
//    (openings
//     ((proof
//       ((lr
//         (((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//          ((0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB)
//           (0x0000000000000000000000000000000000000000000000000000000000000001
//            0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))))
//        (z_1
//         0x0DCBFA69751F87AFE1A3281F2EB46B31506E989C8DA3F6CE2916C26CFC965325)
//        (z_2
//         0x000DFC50DD40ED23A8B0E7F4A6F225856ADDBC994C5560DCDBAA3FD575EBCF46)
//        (delta
//         (0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))
//        (challenge_polynomial_commitment
//         (0x0000000000000000000000000000000000000000000000000000000000000001
//          0x1B74B5A30A12937C53DFA9F06378EE548F655BD4333D477119CF7A23CAED2ABB))))
//      (evals
//       ((w
//         (((0x064C69B93B48BC30234839A5621DAF940776959F9581B36F12619B3AECAA613C)
//           (0x1E7A312BA9FFB127637A8E63BCAB8E6563FA5FAD8B3B6253DEC5AAF1503194B5))
//          ((0x02467B0C17F1027D6099E8BB8788245B06E9658A14E0C4C5F1EBFF71BCB073B2)
//           (0x2784700DF37C92519B7F53DB9409D8B6FB71FA6DD6E7177581C6B96CA3B949E5))
//          ((0x2C2253B3E09227E4FB3C521B12BCA26C145662BF197D56B82F7A3DAABFDE3375)
//           (0x32DE48A34CA7BA4B0C73107744E08BA186112231F10415318A0DBE71B69FFA57))
//          ((0x354CD42B8315850EE7A9C924566F90E4097C0122FD412DDDA5967E3087F0D1F4)
//           (0x1689C7DC0881D8C5F78A20314DBB799C2452FC4094084E6EBE1C009057EF8501))
//          ((0x15B0FC3AFF7ADFFD3388CC86193583FB5488CB60F93550607013E8A2C73F3EDC)
//           (0x044194259118AC88830FBF331A3ECA9BD771E37B95ED92C018CDE39150AF4953))
//          ((0x2A5A98C7FA59BCC21C00B6CAC434E44655DC0446890DA53B48C14F380BBF22E7)
//           (0x365116240142D141A5E7216BB9097193999FF1CD0D4CCAB5F2155F56A7844A7A))
//          ((0x0CD63AB7E233082D6BA631EAA7F5638533DBE86AA8C2D5FE00548EC4563F09E8)
//           (0x1DCCF404F71A40938DE503B1EFFEBF42487D7CB6DB3801A5C31E97D43E6361A6))
//          ((0x0B0891AE1A3B504A1837C6B710CFC33E18F15E9480CB1D66AAFB21F1713E81F6)
//           (0x2054985C2540C51D96010FF8D6923AB5F69FB83BF06D509D9111BBF0F09BBE52))
//          ((0x1576395D0042A58D4D6AEC1A84AF666598BB5C4BA605126EB1EF29D24FB2587E)
//           (0x39E54DE7796BAC7D93C7D114E4E160437BD734CF3812F49527E9CC0B26F89A97))
//          ((0x1676956FBE19C4F3EC162EDC27E9F858A7A6FE11806F4B1B6809E180168D72F3)
//           (0x3979D8644983CF94779916AF372B73F0D21586777F183F727B206A9D35FF6478))
//          ((0x268AAA4520266766ACF3F2FD19A818F374E80254183DD52189B9163C3E75A627)
//           (0x0628FE6ED0746BDA11C9CB189354716CD0D56A828B5069F051F193FEA67C4E66))
//          ((0x27AA599C320CD4FE39B38F759D5833EDA98DDD6A76E945A7FED71A805B60532E)
//           (0x030F1A455FDC020CCE2461097B3A300D3DCA5C751816C6C9D17C7FB088877AB7))
//          ((0x1E2E0CDE4A06CEE7C32059369E1E9A2D52C67E772388032047B8001D73681E2E)
//           (0x37D9BA0369AC741292A1CDED36A2643D3389B0B49E914CA4D3490A4AE5626DD9))
//          ((0x280FE934EF475A73E7666CD787B92D33FD6D98CC6794E8538C2D24FA8DA7E3CC)
//           (0x2EAE317C4461F960E11ACEB9E4695A0CCD23435C5EB86B249AAA73842C697DC3))
//          ((0x260EB6A1D890B71B8AB169A035A0C25A2A5B6A18DF7E1517E3BD427871382DD8)
//           (0x0F1701A9E805C278941BCB76A4F68A4C79A2ACFAEBAF18C457A584F270E29469))))
//        (z
//         ((0x151EE97CA8D2A2C9A9E9542BC82C0D06182529C02EBEBA884FC81397E5157171)
//          (0x1F2231AE6F776DB9C935F68684DED904CD2F47F26049A30FE3CE3F2C9B3CD95D)))
//        (s
//         (((0x24E7AB95462CA37FB5FA7921793EB98C8AC5E22AB94970B9E04BD6D6A0944FA8)
//           (0x178201A8AA99117176923B35C91922B2EBB8EE7E5AE8AC8EAA413B12A56A7C2C))
//          ((0x3D5F054122358E51073C6E1A533D9376455C3F29F99EE242C4EA9FDAEBA2EB16)
//           (0x051661120101B86DEC19AA71223F176E6CFBA0AD760E429C36B85324E1B2F5DC))
//          ((0x3C1616AA99A19CC93C89AF72312484043041FCB8E03D3CE598C5206F982D3635)
//           (0x2E155DA010B2B9F1779CB60A8B93997A80549EE8F02C5E8BEAB45F386A1E95F0))
//          ((0x09ABCCF136D6470878DB839AADA02654172C9116537F3F8BFB28C6809DFABE37)
//           (0x1CFBBD55500252629A5429FE4456218520062A0A6C048431C276602BCC360DE0))
//          ((0x1C492DEB5F754E486A77A2AD2272097FDD410C93AD06C61953917AC00E23DF6B)
//           (0x2436BADFC8B07A229721D9DB5DA6CA70B6767E9D689896B6F3BADDF4EA2B80EE))
//          ((0x10D3408AFC5B4214DDEA3CDE7BD279500283FF0D92890D60D5AA1E883A8DC9B5)
//           (0x1AAD752BA76A6DC82FDE0C61D1E774B16BC54D430701EF6FD7192501281644C0))))
//        (generic_selector
//         ((0x0AB264ABCF5D6768FA0A4862922B3E319AFB4179CE3FE8E5F219A7E1AFC871CD)
//          (0x089AF17A033C472FED3CD53EDFC8A15BBD6906E43C314B68F60604243D0980E5)))
//        (poseidon_selector
//         ((0x39A0A70ADE8B5AB4CEFE48DD5B2F8FBB1945F52BD823A366DFBDE7C3D994CC9A)
//          (0x17756EE6A9147D2C49E5FAE89CAE52D5E82407757858264DC32BB5114206A196)))
//        (lookup ())))
//      (ft_eval1
//       0x1E73CA676452F6A483B48C8D0FD87CFED79DC7109851E63315C59BF987A8F612))))))

/// Value of `vk` when we run `dune runtest src/lib/staged_ledger -f`
/// https://github.com/MinaProtocol/mina/blob/3753a8593cc1577bcf4da16620daf9946d88e8e5/src/lib/staged_ledger/staged_ledger.ml#L2083
///
/// The file was generated this way:
///
/// let buf = Bigstring.create (Side_loaded_verification_key.Stable.V2.bin_size_t vk.data) in
/// ignore (Side_loaded_verification_key.Stable.V2.bin_write_t buf ~pos:0 vk.data : int) ;
/// let bytes = Bigstring.to_bytes buf in
/// let explode s = List.init (String.length s) ~f:(fun i -> String.get s i) in
/// let s = (String.concat ~sep:"," (List.map (explode (Bytes.to_string bytes)) ~f:(fun b -> string_of_int (Char.to_int b)))) in
///
/// Core.Printf.eprintf !"vk=%{sexp: (Side_loaded_verification_key.t, Frozen_ledger_hash.t) With_hash.t}\n%!" vk;
/// Core.Printf.eprintf !"vk_binprot=[%s]\n%!" s;
#[cfg(test)] // Used for tests only
pub fn trivial_verification_key() -> VerificationKey {
    use mina_p2p_messages::v2::MinaBaseVerificationKeyWireStableV1;

    let mut cursor = std::io::Cursor::new(include_bytes!("trivial_vk.bin"));
    let vk = MinaBaseVerificationKeyWireStableV1::binprot_read(&mut cursor).unwrap();

    let vk: VerificationKey = (&vk).into();
    vk
}

// ((data
//   ((max_proofs_verified N2)
//    (wrap_index
//     ((sigma_comm
//       ((0x115A9B9F91775251162313F872CA108495C1EA4C2EB96AFA011657409EC0F654
//         0x3DB545AC7283A12C92D300F3DCA81B517234DBD561E2065D9EB21FD9C2AC27AE)
//        (0x003A95AB0B130CFBA97901BFE78413FC41173E964404CC27768FCEF5C77B4F78
//         0x09F9FF9840CFAB53E923E813780015C305EB94F75552F6D733546D4B85634F88)
//        (0x0DC57926C1C873E53F3FDC8E8704C007DF4B03BE4682CD019B95FC535830C1FC
//         0x045985BFA192EE029CFC2E9D3D22C3F9082B186328CFC6D468C6E1AADBAE3F35)
//        (0x2E2BF21EB30B09B09DD1EFC101D38B6E2AD3347FD314DADBA2207B96C67F99F4
//         0x271CFF2031431A7F88D431D6B7888688944273E28AF7D2068344B3441EE0D971)
//        (0x3892405C3A2B9221C1F386792E2BD4D5D423CCD89044254BA2EE414A817D91D5
//         0x2BC2DBFC1A9B39DC36DFA83CD0EAAA8B941B036ED01E85E92C05E0BE7456B37A)
//        (0x051570E234C491B18EC0B7744443F7A356FD20B0F96E9C22E1729FD661E40FBE
//         0x3DFD707E18CE3376567BC105B7D351B9A5FDF17957694C51C299D6EE981EB53F)
//        (0x29B82A71025C1C4897853F073185F3D3B2EC31459C4B3E5C4BF8FF139F5B286D
//         0x30D72BA2AEB639E01ED33429248975718CEB93E24F78782EEA0035BF86F703F3)))
//      (coefficients_comm
//       ((0x3AC2F5534899799DBCD68518DBCAF7152BB7F8ACE7861D1F425F00BB9547861A
//         0x1BAA259769B1A8FE140011D7840D455B156DC52A901B0E378B7461848078D798)
//        (0x0BA56F62AC687695AB5C1D7057BC6BFEA6FFE80744DFA0913E9B288ACBA491AD
//         0x3D326698D437970DEF4C4090721A0BB9F99082063CD31DF5BD5B16491207AE72)
//        (0x3330EF277DB065EF71343CA82629D152AF8D228CF5080D395C3A1E13211B7CE7
//         0x1083F555EC5FF08F35AE8B2525855FC92CD96FA017DFC047B87DAB67E376B6E5)
//        (0x106AD7F3F7D0649F73804C7E1377C232D20F708167D7B28B93FE9D87991BE775
//         0x343AD3173A552B38B0A33E7DC07A6A9634FF002939C34C9EA3541E76B850A9E3)
//        (0x387E83BABFB643DDF21B7BD6DCE44A8FB8A84323F0F398633754D4DB798EA425
//         0x0974687FFDF09A544152612106ADE816F4F6715495522BD81D65842909708305)
//        (0x371294342F117997B1C65767345FD38606A35F2096D60A5C9B0AF11BDDB1C1C8
//         0x0EC9F56A9F5BF2A8D6C4C088EDE4DC910A1F071EEDCF33B60AD4F7CD5D655104)
//        (0x12ADCC7CCF950D0E60D1DD63C9BD168D44F97EE0D84139BAC3EA4A5763FF4BC4
//         0x39F2B35DBC76730621A3EDF16B395870F1BD13EA9C0AC0C280080BE155613C7E)
//        (0x2ECF70CD1034B62CCFE4079AE0368A233514AF64DB43F47A265EC41916865FE9
//         0x0565120D5B7972C41210196B2166AFC94C29198038FD3CA3B29CC350903CA2F2)
//        (0x1862C2A4FE4A2AD87A55608474B6A3DC9D93728315F0A580BCAEF6667372E3D3
//         0x1F722E3DE5FF8C88A069D64A9F29C8EE81A54FFA0A65C6BCF84B4CB43CE7A77B)
//        (0x397AFEAC5F774219780D7523AE74B52BB36E18028ED052676908340D9B1A04AA
//         0x379011258548CC7FD544C019FE6A48656714668B3574989704C05B9764117EE2)
//        (0x20C9416BFFC7DDC233CD15FC0A1BCFCCE1539A63E6B9213E9812AF6192D56A16
//         0x30E55D7161F26509958DB7523E4665781F246135E3AEA3031336B825E1326FC9)
//        (0x333B91203AEDA652F2178E0C2467AE3C4A15A0EADB9FDA9DD1A6F4226EFBE2FC
//         0x040739E4E5091ED5097D7FCA864CD244EE9D2493BF34828D69C433E57673AA51)
//        (0x1143CBA73A08D58BE6114A1C5DCD81194885360A27FABAE9608D7AB43BDFAF8B
//         0x0902D7E807E44BB9D8AAF906E938BEBBECA30D4F41A995173FAED2334C30A18F)
//        (0x00CBAAFAB8238BB60A77CC976568BB49EE9789F1088E1D85E7CDD60CD64F1BF2
//         0x1CECA45EEEF316BFFFF4AE806F2CF19AB183D948D4BC069439DDF53A1375D20C)
//        (0x0D3CBE9A5B7A2E291AB8834ACECA59FC88DF5EC8A6745047A84F6D66F7BE19ED
//         0x1D3FA4DD7F47437A9A06EB7BCD7E15E552D3B7652E57429854826F8EBE39E5DA)))
//      (generic_comm
//       (0x38B8C790877BE60FFBDBFEF30A6ECE76BF3B193777D1F5FEECE38E1CB26FD059
//        0x112D6B9D5375B2D7CA3F7DFD23829406FAE8E97746FC29EB4847CD5C4B5175D7))
//      (psm_comm
//       (0x2F40E2225F0799CBD4B0CA57C6BDEE634041F9579F97AA729F3A3BA8C14B21E9
//        0x0E2434BEFB78370542AC791EA239635D8856C215D25F152E1D077C49CD1F552D))
//      (complete_add_comm
//       (0x008FC7F203536C4FCFC1E09FFA50FA30247D1E0781774C613B7315C3C68C0D0D
//        0x38DA093708AD8CDEB75CA58971FB773AF167EB04D9402678BB861E6E0518208E))
//      (mul_comm
//       (0x3EFA1F93096F3046C84B5911B003B81823929B437C6ED65F9926455370936729
//        0x3FF7C9E9503815A15ECA37D478067E44C1E0CFBDDA4A00ED8F39F706839413D2))
//      (emul_comm
//       (0x2663A3B30707CB71702E64CE42A15FC9D7FEB56D0AB6617F0F08BDF86DA8B5F1
//        0x02A0951D13AC3DBA4BCBC745C20282F379699BD939F6CAEF4830BAF1A9147B77))
//      (endomul_scalar_comm
//       (0x3D41F1D97C6C0C57975D6E2C82E61C203FB3B2B45BD952DA3141AD8535C5D99D
//        0x3E8E506C3999F9DB8848E1E2236EC26316F06B551C841D4B562F07E152ECE030))))))
//  (hash
//   19499466121496341533850180868238667461929019416054840058730806488105861126057))
