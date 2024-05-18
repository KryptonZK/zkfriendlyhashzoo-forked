use super::rescue_prime_params::RescuePrimeParams;
use crate::fields::f64::F64;

use ff::from_hex;
use lazy_static::lazy_static;
use std::sync::Arc;

type Scalar = F64;

lazy_static! {
    pub static ref MDS8: Vec<Vec<Scalar>> = vec![
        vec![
            from_hex("0x14732c45f87debc7").unwrap(),
            from_hex("0xe6d99297dc2f523d").unwrap(),
            from_hex("0x6fa73839a1806877").unwrap(),
            from_hex("0xd5da6d851107610a").unwrap(),
            from_hex("0xbf2ab278d283311b").unwrap(),
            from_hex("0x0006e902841eb1a0").unwrap(),
            from_hex("0xffffffe4221a6ca5").unwrap(),
            from_hex("0x00000000000ea920").unwrap(),
        ],
        vec![
            from_hex("0x02747c7c1544ac11").unwrap(),
            from_hex("0x95c7029b3afb3fff").unwrap(),
            from_hex("0x530bdf5887ae8b34").unwrap(),
            from_hex("0x5a94eebaeffbe47d").unwrap(),
            from_hex("0xada8f565ac4764c9").unwrap(),
            from_hex("0x0dfdb615505c04b5").unwrap(),
            from_hex("0xfe7d069b2a1c8a21").unwrap(),
            from_hex("0x000000bc1155b0a4").unwrap(),
        ],
        vec![
            from_hex("0x23647df23cdfd9c0").unwrap(),
            from_hex("0x86d3d1e66ff6eab3").unwrap(),
            from_hex("0x331449248936374a").unwrap(),
            from_hex("0x4de4709d4a8e10eb").unwrap(),
            from_hex("0x7d5fe42ba6740593").unwrap(),
            from_hex("0x0681e4f6ff1b99a6").unwrap(),
            from_hex("0x47aaf5014a6e7182").unwrap(),
            from_hex("0x0942383f8f66e2a0").unwrap(),
        ],
        vec![
            from_hex("0xcee197006d237495").unwrap(),
            from_hex("0xa877f59b6530b358").unwrap(),
            from_hex("0x4cd08775fe49a04a").unwrap(),
            from_hex("0x9ed7da411f550715").unwrap(),
            from_hex("0x4109b6963c8fba3d").unwrap(),
            from_hex("0x2dd725d1ca3721fb").unwrap(),
            from_hex("0xf38ee34581a076bc").unwrap(),
            from_hex("0x3a8e51fb87a5ddc5").unwrap(),
        ],
        vec![
            from_hex("0xe45012c1df25dee3").unwrap(),
            from_hex("0x85413d954de9d3bc").unwrap(),
            from_hex("0x4e6c6c8ea7e24d04").unwrap(),
            from_hex("0x717a3405f77835cc").unwrap(),
            from_hex("0x4f24221862041750").unwrap(),
            from_hex("0xceb178cc3bcbcdd7").unwrap(),
            from_hex("0x03757f5e7f40038b").unwrap(),
            from_hex("0xb53cf4cd1685e1e4").unwrap(),
        ],
        vec![
            from_hex("0x6cb6558784f7959c").unwrap(),
            from_hex("0xcbda2e10da4d4ec9").unwrap(),
            from_hex("0x2fb8571a6fdb48bd").unwrap(),
            from_hex("0xd3e5fceeba32cf14").unwrap(),
            from_hex("0xa5afdcb6f6e326c8").unwrap(),
            from_hex("0x7cc126270ff2c210").unwrap(),
            from_hex("0x556f0c39634db7fd").unwrap(),
            from_hex("0x4bf119430c8962fa").unwrap(),
        ],
        vec![
            from_hex("0x68f9a9ca9284d01c").unwrap(),
            from_hex("0x76c496b60ecaa92e").unwrap(),
            from_hex("0x0a375191b0f5f972").unwrap(),
            from_hex("0x6160ec419d879f1d").unwrap(),
            from_hex("0xba5f8a483378d5e7").unwrap(),
            from_hex("0x93d1918084d5e022").unwrap(),
            from_hex("0x0789e520476b7040").unwrap(),
            from_hex("0x5eee80c01078c7e2").unwrap(),
        ],
        vec![
            from_hex("0x16e27408670433fa").unwrap(),
            from_hex("0x39a1f0a8bcd9672f").unwrap(),
            from_hex("0xeef802e2e2248e30").unwrap(),
            from_hex("0xf7466d4fecfd366d").unwrap(),
            from_hex("0xa6d49daccf1a2c47").unwrap(),
            from_hex("0xda29d438ef17e5a1").unwrap(),
            from_hex("0xe55f9ad24dff5f9a").unwrap(),
            from_hex("0x62df1e5f00cf2ebe").unwrap(),
        ],
    ];
    pub static ref RC8: Vec<Vec<Scalar>> = vec![
        vec![
            from_hex("0x48dc4d4899f9ea51").unwrap(),
            from_hex("0xf9ae2fdfb671c2a2").unwrap(),
            from_hex("0xd5370967c6e931fd").unwrap(),
            from_hex("0x68c8605b1712cb98").unwrap(),
            from_hex("0x09cb4f4e60c7e35c").unwrap(),
            from_hex("0xcc411fd2ecb7184e").unwrap(),
            from_hex("0xd8ac59b4dc74caac").unwrap(),
            from_hex("0x7a81fc5c89849737").unwrap(),
        ],
        vec![
            from_hex("0x88d6000f340065ab").unwrap(),
            from_hex("0x90360121bb76ba19").unwrap(),
            from_hex("0x4533eb127a8d352b").unwrap(),
            from_hex("0xf6c6461666615df2").unwrap(),
            from_hex("0x5f9a5d7ddaf81433").unwrap(),
            from_hex("0x7c37be07d68ba565").unwrap(),
            from_hex("0xec99ad2e14346553").unwrap(),
            from_hex("0xc451c6c465c7fe5b").unwrap(),
        ],
        vec![
            from_hex("0x3503bcd9e97dedf2").unwrap(),
            from_hex("0x2efb8e683dc769b9").unwrap(),
            from_hex("0xdf5be4db53ec2f57").unwrap(),
            from_hex("0xe24115bb13b12dba").unwrap(),
            from_hex("0x78703700acdb1e6c").unwrap(),
            from_hex("0x06ea36151155e79d").unwrap(),
            from_hex("0xf54e5231f803cd33").unwrap(),
            from_hex("0xd0142049d496aa65").unwrap(),
        ],
        vec![
            from_hex("0x09d1cac488a5e34a").unwrap(),
            from_hex("0x5d537898181b71ca").unwrap(),
            from_hex("0xdec6f49134643525").unwrap(),
            from_hex("0x1b48b1ca07a6b18e").unwrap(),
            from_hex("0x82ac738f89fb1bdd").unwrap(),
            from_hex("0x0908cc91dfee154f").unwrap(),
            from_hex("0x8af85a7124835202").unwrap(),
            from_hex("0xc7d790cef7bc0ea9").unwrap(),
        ],
        vec![
            from_hex("0xc5e156a585efbce8").unwrap(),
            from_hex("0x16fb321e2803c594").unwrap(),
            from_hex("0x629a135b4cbc4b90").unwrap(),
            from_hex("0xa47c19f17c6122f3").unwrap(),
            from_hex("0xf73fb577343a393e").unwrap(),
            from_hex("0x37426c75c7c0acd7").unwrap(),
            from_hex("0x26f6226e0bcd1599").unwrap(),
            from_hex("0xcfcf1f71b232a6b0").unwrap(),
        ],
        vec![
            from_hex("0x633749756cff8d66").unwrap(),
            from_hex("0x627adb8d9faa99e2").unwrap(),
            from_hex("0xa9463bd5d97b5889").unwrap(),
            from_hex("0xb1b24a08d03e4450").unwrap(),
            from_hex("0xf19b380c0c2b9af2").unwrap(),
            from_hex("0x27225943c47577c7").unwrap(),
            from_hex("0xdbb5504da65f774e").unwrap(),
            from_hex("0xd8e80153dc9d71d6").unwrap(),
        ],
        vec![
            from_hex("0xdd5016c3790109a5").unwrap(),
            from_hex("0xcbfa7d1b0514b124").unwrap(),
            from_hex("0xc35bd2fc284bdcbe").unwrap(),
            from_hex("0x27b011e5661fee67").unwrap(),
            from_hex("0x44a0d347e872f173").unwrap(),
            from_hex("0xd2b5e0b8f83e6f53").unwrap(),
            from_hex("0x697eee98a6be042a").unwrap(),
            from_hex("0x59e3b9a6424b686a").unwrap(),
        ],
        vec![
            from_hex("0xf92748d77629c057").unwrap(),
            from_hex("0x56063f1ce396cfab").unwrap(),
            from_hex("0x7f16c812191adfec").unwrap(),
            from_hex("0x83bf18af10987909").unwrap(),
            from_hex("0xde782cfdade79980").unwrap(),
            from_hex("0x41c6d4cbe4854f97").unwrap(),
            from_hex("0xfdc98bfe3724684a").unwrap(),
            from_hex("0x7a199803e43a9295").unwrap(),
        ],
        vec![
            from_hex("0xfcc42dfaa9787ba5").unwrap(),
            from_hex("0x8b37afc23e2b8bbd").unwrap(),
            from_hex("0x52e08d84e86667d5").unwrap(),
            from_hex("0x443c2001b7e37af4").unwrap(),
            from_hex("0x3226da2e2816de96").unwrap(),
            from_hex("0xe7f7d866987ef7ed").unwrap(),
            from_hex("0xcaa88bb146922f16").unwrap(),
            from_hex("0x53ca795efd843970").unwrap(),
        ],
        vec![
            from_hex("0xa15389d8d9dd9a04").unwrap(),
            from_hex("0xb35c9691c9debd23").unwrap(),
            from_hex("0x9b5356e91d08be51").unwrap(),
            from_hex("0xc728a3ffca5b56a1").unwrap(),
            from_hex("0x5c4622e25a450c6a").unwrap(),
            from_hex("0x1d3c0838581c96b9").unwrap(),
            from_hex("0xac8f7185c6973979").unwrap(),
            from_hex("0x706ddfad2aeb6c91").unwrap(),
        ],
        vec![
            from_hex("0x99f05e66ef2138e7").unwrap(),
            from_hex("0xc967c7e09f5b8d77").unwrap(),
            from_hex("0xb01e5c6cd3434d00").unwrap(),
            from_hex("0x42b7280044f20d40").unwrap(),
            from_hex("0x48446bfa283c9d01").unwrap(),
            from_hex("0x52dcdc2d37ba4ee7").unwrap(),
            from_hex("0xf58362fe1b35f4c6").unwrap(),
            from_hex("0x25e9a1dde599478b").unwrap(),
        ],
        vec![
            from_hex("0xc1426eff7b91f8aa").unwrap(),
            from_hex("0xddc866f470fdb6c5").unwrap(),
            from_hex("0xf172cd604c912846").unwrap(),
            from_hex("0xf129c49dfcdd3ebb").unwrap(),
            from_hex("0x47cdb7283d89477c").unwrap(),
            from_hex("0x332f19abc517fc6b").unwrap(),
            from_hex("0xe080445bd93d1ae6").unwrap(),
            from_hex("0x59f77cc2fb1fa50b").unwrap(),
        ],
        vec![
            from_hex("0x815acf2b71c206f5").unwrap(),
            from_hex("0x735dde3098e9aedd").unwrap(),
            from_hex("0xd58f37ecea7571d9").unwrap(),
            from_hex("0xe98697abebe24454").unwrap(),
            from_hex("0x662bf4f9941897c5").unwrap(),
            from_hex("0x4c59760b71c2ebab").unwrap(),
            from_hex("0xeb5bc3da23eb4bd1").unwrap(),
            from_hex("0xd1f11656eb857389").unwrap(),
        ],
        vec![
            from_hex("0x7fa28b567cf63297").unwrap(),
            from_hex("0x18aab8654c1f7de1").unwrap(),
            from_hex("0x467df2d6691d51c5").unwrap(),
            from_hex("0xd1478088a7464cde").unwrap(),
            from_hex("0x3b617a4ce55c301f").unwrap(),
            from_hex("0x5bbae0ffffa2eef7").unwrap(),
            from_hex("0x090b1b72fd19db1c").unwrap(),
            from_hex("0xc85093e3704e2144").unwrap(),
        ],
        vec![
            from_hex("0xf4eee76f961854bc").unwrap(),
            from_hex("0x0d167835b8567e60").unwrap(),
            from_hex("0xfbcb296cb62f6ec7").unwrap(),
            from_hex("0x8eae0ce7490b9823").unwrap(),
            from_hex("0xba0e91a58f44f539").unwrap(),
            from_hex("0xdbae476783c5350c").unwrap(),
            from_hex("0xfb53b08a24e13633").unwrap(),
            from_hex("0x0df795a5e4dbd099").unwrap(),
        ],
        vec![
            from_hex("0xfd6acb60aafcc490").unwrap(),
            from_hex("0xa4d91b10297c702f").unwrap(),
            from_hex("0x091d6f7e335a37a7").unwrap(),
            from_hex("0x3e211705f420582b").unwrap(),
            from_hex("0x8909707610d1593c").unwrap(),
            from_hex("0x4fca690bdb6bb019").unwrap(),
            from_hex("0x560153759eb732bd").unwrap(),
            from_hex("0xe98caa70d54fcc9e").unwrap(),
        ],
    ];
    pub static ref MDS12: Vec<Vec<Scalar>> = vec![
        vec![
            from_hex("0x1d4432c2c62b8560").unwrap(),
            from_hex("0x9bc11561d6440acb").unwrap(),
            from_hex("0x202ca9ebe5cceb64").unwrap(),
            from_hex("0x9bfe2a4f0c017c2a").unwrap(),
            from_hex("0x6f1ff66150e7e72b").unwrap(),
            from_hex("0x99c7056e7a4e495b").unwrap(),
            from_hex("0x3671223a0ae084fd").unwrap(),
            from_hex("0xee9d983091e3d5a9").unwrap(),
            from_hex("0x021e37506702caaa").unwrap(),
            from_hex("0x63f74568eb8a4c10").unwrap(),
            from_hex("0xf6c4b0a72dba2fb7").unwrap(),
            from_hex("0x00000000898036b0").unwrap(),
        ],
        vec![
            from_hex("0x2ec0835c6c55ca7c").unwrap(),
            from_hex("0x4cc36b4624116cae").unwrap(),
            from_hex("0x6b833e9b3184f367").unwrap(),
            from_hex("0xc4925e08b239ff38").unwrap(),
            from_hex("0x40946583f303b927").unwrap(),
            from_hex("0x4c6292ccf81b0176").unwrap(),
            from_hex("0x2edc329316f945c7").unwrap(),
            from_hex("0x1769b9de2beb36f5").unwrap(),
            from_hex("0x7385d036486bcb5f").unwrap(),
            from_hex("0xf4f63e1de4711088").unwrap(),
            from_hex("0xd80e5636e790da47").unwrap(),
            from_hex("0x409f2b674968e8b6").unwrap(),
        ],
        vec![
            from_hex("0x2388d9365f8d086e").unwrap(),
            from_hex("0x95ca47ec855e4eb2").unwrap(),
            from_hex("0x05ab2b5356e05b9e").unwrap(),
            from_hex("0xc0d2cd7eab963979").unwrap(),
            from_hex("0xf7cd4dcbe23adfad").unwrap(),
            from_hex("0xc95e3c5ffb05edaa").unwrap(),
            from_hex("0xa8f8edee8da2b931").unwrap(),
            from_hex("0xa6b5af17a0f7e23f").unwrap(),
            from_hex("0x0d8e93bb5b5990d6").unwrap(),
            from_hex("0x2c3a8d613a13810f").unwrap(),
            from_hex("0x404442655843e95b").unwrap(),
            from_hex("0xf5475b511f11afc9").unwrap(),
        ],
        vec![
            from_hex("0x97d4f25db7d4bae3").unwrap(),
            from_hex("0x7ea6ca3c47cfd890").unwrap(),
            from_hex("0xba8b270db132aca8").unwrap(),
            from_hex("0xbfe968a65d720a56").unwrap(),
            from_hex("0x56ad2192d27a1592").unwrap(),
            from_hex("0x2b43ced6084ac90f").unwrap(),
            from_hex("0xf1528542c1c708f1").unwrap(),
            from_hex("0x328f12e8482a2dc5").unwrap(),
            from_hex("0x917ef019c09894b4").unwrap(),
            from_hex("0x386fba1f35a6ed31").unwrap(),
            from_hex("0x7aca524ab57dfcc1").unwrap(),
            from_hex("0x84842e9461432199").unwrap(),
        ],
        vec![
            from_hex("0xd945a3b972e6545e").unwrap(),
            from_hex("0x044c6c8187d1db7f").unwrap(),
            from_hex("0xdf17bb7a8b70a3d4").unwrap(),
            from_hex("0x4ab87e3a7e93ddd6").unwrap(),
            from_hex("0xd28b1641cfb56a6c").unwrap(),
            from_hex("0x5c6e359fb8727a31").unwrap(),
            from_hex("0x5b87beea92e0b2ce").unwrap(),
            from_hex("0xd4bfd68ca6d159a1").unwrap(),
            from_hex("0x254b361e05918ecf").unwrap(),
            from_hex("0xdcc27d13db8a5725").unwrap(),
            from_hex("0x2666d2ce353f36e3").unwrap(),
            from_hex("0x70e84eb1230e409d").unwrap(),
        ],
        vec![
            from_hex("0xcbd41ae089895ff9").unwrap(),
            from_hex("0x70ba27f427fc468a").unwrap(),
            from_hex("0xebe593c21d3d5084").unwrap(),
            from_hex("0x284d3f173d043bc0").unwrap(),
            from_hex("0x9b0451ddedf53a94").unwrap(),
            from_hex("0x4b9d26f247444217").unwrap(),
            from_hex("0x8787f807bcbf7469").unwrap(),
            from_hex("0x35765054162bc210").unwrap(),
            from_hex("0xca4c5ceede976ebb").unwrap(),
            from_hex("0xa6768e87e8400447").unwrap(),
            from_hex("0x732ced96bdb4c4aa").unwrap(),
            from_hex("0x27af50126787e270").unwrap(),
        ],
        vec![
            from_hex("0x444a0e7d460b2987").unwrap(),
            from_hex("0xb9adab858ff7f4a2").unwrap(),
            from_hex("0x2bfb348d94abae16").unwrap(),
            from_hex("0xda9ed3e85a6cfea2").unwrap(),
            from_hex("0x08a2d39045f82546").unwrap(),
            from_hex("0xc305f534f614e394").unwrap(),
            from_hex("0x479b7371a0dfac64").unwrap(),
            from_hex("0xf2073fc4629c5419").unwrap(),
            from_hex("0x8a0574193bb44f01").unwrap(),
            from_hex("0xbd64db499b136800").unwrap(),
            from_hex("0x003467f37d001520").unwrap(),
            from_hex("0xae840a2fa7935fae").unwrap(),
        ],
        vec![
            from_hex("0x2ecb3a5a4e76cd9d").unwrap(),
            from_hex("0x7b5253aa4e5d296e").unwrap(),
            from_hex("0xd9904d2d6d5d4357").unwrap(),
            from_hex("0xb7c84148102fc9a1").unwrap(),
            from_hex("0xa89d7544c75dd629").unwrap(),
            from_hex("0x13d0c8233d513e1c").unwrap(),
            from_hex("0x37faacb3482248e5").unwrap(),
            from_hex("0xccda3c18931e54cb").unwrap(),
            from_hex("0x9f1cbddcf5524b2f").unwrap(),
            from_hex("0xa818c4e3203b2c20").unwrap(),
            from_hex("0xf0b20bd7905d52c1").unwrap(),
            from_hex("0xcb5f2eb35fc48000").unwrap(),
        ],
        vec![
            from_hex("0x4b3e156b5cc2b9b5").unwrap(),
            from_hex("0xc514abe21838143c").unwrap(),
            from_hex("0x496c10024f7f89f7").unwrap(),
            from_hex("0x0e28687dfb263e48").unwrap(),
            from_hex("0xc69c1c8c68f3cab6").unwrap(),
            from_hex("0x6ca309ef3ee85638").unwrap(),
            from_hex("0x82f61a93d57a9534").unwrap(),
            from_hex("0x4f538d204147839c").unwrap(),
            from_hex("0xd520ff01048b2e24").unwrap(),
            from_hex("0x3955de4f89b618c4").unwrap(),
            from_hex("0xe8f1478786466178").unwrap(),
            from_hex("0x9b27d3246d3987b9").unwrap(),
        ],
        vec![
            from_hex("0x910bc2a89fd955b1").unwrap(),
            from_hex("0xa8525755c08ebda7").unwrap(),
            from_hex("0x938876200811379b").unwrap(),
            from_hex("0x5f8bcf49f0602799").unwrap(),
            from_hex("0xe3c8a72fa5132910").unwrap(),
            from_hex("0xbd43552e28503732").unwrap(),
            from_hex("0x238048495bd93cb6").unwrap(),
            from_hex("0x3c0fdb9eefab3cd4").unwrap(),
            from_hex("0x3ac9701d5b6038e0").unwrap(),
            from_hex("0x1ce14d168b57b6ef").unwrap(),
            from_hex("0x1c6a38085ce81245").unwrap(),
            from_hex("0x5edc8b104a9eb19a").unwrap(),
        ],
        vec![
            from_hex("0x4ec29ed04b4c4964").unwrap(),
            from_hex("0x70b304c1a0fc291f").unwrap(),
            from_hex("0x88c905f3ded7137f").unwrap(),
            from_hex("0x1b35910e8342a387").unwrap(),
            from_hex("0xd40a1da0ff916ef1").unwrap(),
            from_hex("0xf9ca73079f019da1").unwrap(),
            from_hex("0x01033e3e72e6ce39").unwrap(),
            from_hex("0x7b81d19ba52bcb25").unwrap(),
            from_hex("0xeba6ca0474260fca").unwrap(),
            from_hex("0x58fe79ae4c0f2cf5").unwrap(),
            from_hex("0xc125d8de133dd49f").unwrap(),
            from_hex("0x4c67085227851f30").unwrap(),
        ],
        vec![
            from_hex("0x8f32049c8a4e0020").unwrap(),
            from_hex("0xf05daf4764cf2933").unwrap(),
            from_hex("0xa029343fe68b9154").unwrap(),
            from_hex("0x64e63504883c12d4").unwrap(),
            from_hex("0x41fe4fe19aa4f6aa").unwrap(),
            from_hex("0x1e713f98a7184ddf").unwrap(),
            from_hex("0xa21e9b8b691b563f").unwrap(),
            from_hex("0x6f069368b627d139").unwrap(),
            from_hex("0x5da04e94bc4258ba").unwrap(),
            from_hex("0xa7decd2f51d2a109").unwrap(),
            from_hex("0xb74cbb64c0b7ce74").unwrap(),
            from_hex("0x4d004d3a724dfe54").unwrap(),
        ],
    ];
    pub static ref RC12: Vec<Vec<Scalar>> = vec![
        vec![
            from_hex("0xdf4a7c2aeaa76b43").unwrap(),
            from_hex("0x36f6146f159448a3").unwrap(),
            from_hex("0x20806950af2cb240").unwrap(),
            from_hex("0xe52bc17cde4a9396").unwrap(),
            from_hex("0x22955641abac882e").unwrap(),
            from_hex("0x1f24251cc7584861").unwrap(),
            from_hex("0x0ee166359dc2f227").unwrap(),
            from_hex("0x84e589d15fe9c8b3").unwrap(),
            from_hex("0xbfbeebeea04d9cfd").unwrap(),
            from_hex("0x0e12626bbef49c65").unwrap(),
            from_hex("0x59c73926c0c09258").unwrap(),
            from_hex("0x090a8b7ab5cba96b").unwrap(),
        ],
        vec![
            from_hex("0x93edc3c90d41a7bc").unwrap(),
            from_hex("0x5c6891eddf5cfe94").unwrap(),
            from_hex("0xbaf99b281bb03ff9").unwrap(),
            from_hex("0x9c2eb2dd6b7eb3f9").unwrap(),
            from_hex("0x1e889fc821a4be09").unwrap(),
            from_hex("0x82002d129c81d374").unwrap(),
            from_hex("0x50297b2f9666b8d9").unwrap(),
            from_hex("0xdc622d7b18fca35e").unwrap(),
            from_hex("0xd110214cb87641e9").unwrap(),
            from_hex("0xee74064efeb7b334").unwrap(),
            from_hex("0x0311f5353a86a3f4").unwrap(),
            from_hex("0x975448f9f7d59930").unwrap(),
        ],
        vec![
            from_hex("0x5df41d8874c695f8").unwrap(),
            from_hex("0x82da97ffe65920ac").unwrap(),
            from_hex("0x580e84993f50682a").unwrap(),
            from_hex("0x12b5ff159b281de6").unwrap(),
            from_hex("0x9c39cc1fbe3afa05").unwrap(),
            from_hex("0x8daf7368680a0f5c").unwrap(),
            from_hex("0xc1679bcd580dd7b0").unwrap(),
            from_hex("0x0674d434e3dff25d").unwrap(),
            from_hex("0xdfbfd639969c6454").unwrap(),
            from_hex("0xd1ebe222c05bf99b").unwrap(),
            from_hex("0xfc8444539e4fa4b2").unwrap(),
            from_hex("0x3e34f988211f5129").unwrap(),
        ],
        vec![
            from_hex("0x820e016a12d1fa35").unwrap(),
            from_hex("0xd952ff35ebd208c5").unwrap(),
            from_hex("0x2f1f7275b141ae15").unwrap(),
            from_hex("0x09294e1238c74824").unwrap(),
            from_hex("0x466aea4707d2d1b9").unwrap(),
            from_hex("0xf2380216df52247a").unwrap(),
            from_hex("0x9bb9643d459c4b23").unwrap(),
            from_hex("0x5a25f0df37bdf030").unwrap(),
            from_hex("0xecc71239a7014b23").unwrap(),
            from_hex("0xaba57ca39ba8e2bb").unwrap(),
            from_hex("0x7ba0e06ee05cb674").unwrap(),
            from_hex("0x1cd6ef3e8e1a8e4d").unwrap(),
        ],
        vec![
            from_hex("0x3cf604a202e65055").unwrap(),
            from_hex("0x0005eb1f7c758f3f").unwrap(),
            from_hex("0x3a8f84225d1a83ea").unwrap(),
            from_hex("0x11095ba466230bd3").unwrap(),
            from_hex("0x71ab78d709010bef").unwrap(),
            from_hex("0x72ef94c69b99e5b4").unwrap(),
            from_hex("0xdbc62d71ff4a119d").unwrap(),
            from_hex("0x4dd056313ea417a4").unwrap(),
            from_hex("0x79ec27cc236fc314").unwrap(),
            from_hex("0xd8e312ad83af2a7c").unwrap(),
            from_hex("0xd8fd14a237f8187b").unwrap(),
            from_hex("0x723a6b7de8e7fc85").unwrap(),
        ],
        vec![
            from_hex("0xb6c00937ffa0ff87").unwrap(),
            from_hex("0xfd1ebf86249d4eef").unwrap(),
            from_hex("0x6a0af5be41ebe1fc").unwrap(),
            from_hex("0x6c88ada5a967a389").unwrap(),
            from_hex("0x0f6e094f796a154e").unwrap(),
            from_hex("0x01f0cbe704014831").unwrap(),
            from_hex("0x623364077f0ec4fc").unwrap(),
            from_hex("0x45776b9eb34215ec").unwrap(),
            from_hex("0x5a07ec086c93391e").unwrap(),
            from_hex("0x4f0b0e5dc84eab49").unwrap(),
            from_hex("0xfbe67d647097a609").unwrap(),
            from_hex("0xb17d4f1db757ef73").unwrap(),
        ],
        vec![
            from_hex("0x2cff5dd2e15b6b09").unwrap(),
            from_hex("0x984cf4b5d2f28e9c").unwrap(),
            from_hex("0xfdadf07472065cb8").unwrap(),
            from_hex("0xc2eb929d0d9bd828").unwrap(),
            from_hex("0xadd3584e85d1e760").unwrap(),
            from_hex("0x1a70d2f530089515").unwrap(),
            from_hex("0x81b6095c2961ec14").unwrap(),
            from_hex("0x18145491fbc7c37c").unwrap(),
            from_hex("0x2e0a379d5a303b49").unwrap(),
            from_hex("0x36c3b409a559d993").unwrap(),
            from_hex("0x062cedee3b5f422d").unwrap(),
            from_hex("0x2b0efb333c1b4ec4").unwrap(),
        ],
        vec![
            from_hex("0xee3d90f29221fb94").unwrap(),
            from_hex("0x512a4ad495a917b5").unwrap(),
            from_hex("0xc3e0ee4e5be42aa2").unwrap(),
            from_hex("0xd1c1f30697b41ce8").unwrap(),
            from_hex("0x4924c0bafe03eab3").unwrap(),
            from_hex("0xa853be4100776cf8").unwrap(),
            from_hex("0xfdb6327314910d0c").unwrap(),
            from_hex("0x084a66bdc4d45872").unwrap(),
            from_hex("0x53d9e5507b940647").unwrap(),
            from_hex("0x0190c823c7dfb248").unwrap(),
            from_hex("0x27fdf46b9d152106").unwrap(),
            from_hex("0x2fc9d067c4cc03a2").unwrap(),
        ],
        vec![
            from_hex("0x9fee6eaaa885c8a0").unwrap(),
            from_hex("0xe6514d5e6bd053f4").unwrap(),
            from_hex("0xa72e17d101192d78").unwrap(),
            from_hex("0x8f6e371c66d76c94").unwrap(),
            from_hex("0x34cd7ba573a2c096").unwrap(),
            from_hex("0x439a7c0d8bf89cf7").unwrap(),
            from_hex("0x4c69c6cdcccb5022").unwrap(),
            from_hex("0x0fe3097b897256f0").unwrap(),
            from_hex("0xdadd08cdb07c6e20").unwrap(),
            from_hex("0x005120ea8ab7c721").unwrap(),
            from_hex("0xd8d56aed1d3b232c").unwrap(),
            from_hex("0x751bec1376b750d2").unwrap(),
        ],
        vec![
            from_hex("0xb8887a180fedceca").unwrap(),
            from_hex("0x660bc126c2c2d6fb").unwrap(),
            from_hex("0xc8e1c3abb2cbd531").unwrap(),
            from_hex("0x1b9dc069a6dd8cb7").unwrap(),
            from_hex("0x264c77da403d20f4").unwrap(),
            from_hex("0x51b72162affc1a40").unwrap(),
            from_hex("0x2eb73a2c66e2a4f7").unwrap(),
            from_hex("0x96de27eedfd8809e").unwrap(),
            from_hex("0x673550c2931904ad").unwrap(),
            from_hex("0x8bef03b956084508").unwrap(),
            from_hex("0x17f9c4fbd53e721c").unwrap(),
            from_hex("0xdc54cadee6558c34").unwrap(),
        ],
        vec![
            from_hex("0x1cd502044efb620a").unwrap(),
            from_hex("0x0067e87c53a94787").unwrap(),
            from_hex("0x6846ea55e04c937e").unwrap(),
            from_hex("0xc921fc38d2b5458f").unwrap(),
            from_hex("0xb6535259e247a66b").unwrap(),
            from_hex("0xec8ba314290144a0").unwrap(),
            from_hex("0x7400b44ed05f4b04").unwrap(),
            from_hex("0x5c075e01fb0be205").unwrap(),
            from_hex("0xe000a30c1c2de0a0").unwrap(),
            from_hex("0xa7e44cd6ee91c152").unwrap(),
            from_hex("0xe62208d413a283d8").unwrap(),
            from_hex("0x9b37682c988a7f6d").unwrap(),
        ],
        vec![
            from_hex("0xc91b07ecd5e520e4").unwrap(),
            from_hex("0xe886e508cfbde663").unwrap(),
            from_hex("0x3a57d6241dfb2b7d").unwrap(),
            from_hex("0xf1235561577a94de").unwrap(),
            from_hex("0xe52b52113f35d62b").unwrap(),
            from_hex("0xafd91d2b649b561d").unwrap(),
            from_hex("0x66403afee8e8c4cb").unwrap(),
            from_hex("0x4303746fb5531e6b").unwrap(),
            from_hex("0x086626a246ee0da4").unwrap(),
            from_hex("0x959912ae6b28ee60").unwrap(),
            from_hex("0xd855ce73157a6a39").unwrap(),
            from_hex("0xe8085cc563759366").unwrap(),
        ],
        vec![
            from_hex("0xa03af941f674d5db").unwrap(),
            from_hex("0x685ec828e76cec2b").unwrap(),
            from_hex("0x4b6776291ebd3931").unwrap(),
            from_hex("0xf418123ad4a424d6").unwrap(),
            from_hex("0x734ec470ef28edc6").unwrap(),
            from_hex("0xc264d009d8d2597c").unwrap(),
            from_hex("0xdca434bd40769c7c").unwrap(),
            from_hex("0x0b481cd44944a9c9").unwrap(),
            from_hex("0x5ea04d088a1d0701").unwrap(),
            from_hex("0xae57661b5af56e24").unwrap(),
            from_hex("0x34fba8c61f95b7fa").unwrap(),
            from_hex("0xeb497ca1f81cd385").unwrap(),
        ],
        vec![
            from_hex("0x6eb26e6a31ad928e").unwrap(),
            from_hex("0x937327d499dfd51f").unwrap(),
            from_hex("0xa4939cbf0b385a8d").unwrap(),
            from_hex("0x608159f3a343a189").unwrap(),
            from_hex("0xd6f05f349cd243d5").unwrap(),
            from_hex("0x0b80feee3073e180").unwrap(),
            from_hex("0xdce9a9ef117a7daa").unwrap(),
            from_hex("0x8ce09765f42f07de").unwrap(),
            from_hex("0x9295e7ca46013110").unwrap(),
            from_hex("0xff52bc22c262edb7").unwrap(),
            from_hex("0x0c3137d856a41485").unwrap(),
            from_hex("0xe616a21fd06d027d").unwrap(),
        ],
        vec![
            from_hex("0x8937d3fdca04dfc2").unwrap(),
            from_hex("0x16c986e2fb382eba").unwrap(),
            from_hex("0x01ee5045c70ef0c8").unwrap(),
            from_hex("0x08ebafa770a6c938").unwrap(),
            from_hex("0xbe3e5a7894d4da8a").unwrap(),
            from_hex("0x6b870bd34e65bb36").unwrap(),
            from_hex("0x841ab26977e20b53").unwrap(),
            from_hex("0xd7d76ff20450f97a").unwrap(),
            from_hex("0xd4ccad9ab88d9755").unwrap(),
            from_hex("0x3f65f37468333171").unwrap(),
            from_hex("0xbaeedd8884d239ea").unwrap(),
            from_hex("0x8dcb991b9ff8a30d").unwrap(),
        ],
        vec![
            from_hex("0x50ebb1bd1f97059e").unwrap(),
            from_hex("0xd24e10305cc29cf2").unwrap(),
            from_hex("0xe55f63fae5f76e0d").unwrap(),
            from_hex("0xb3ce2b562db82712").unwrap(),
            from_hex("0xd4b3423421fed2a7").unwrap(),
            from_hex("0x6c7128887ac5fd6b").unwrap(),
            from_hex("0x3c825c9493166ff7").unwrap(),
            from_hex("0x716956326df30e1b").unwrap(),
            from_hex("0x58487434d42f9c1e").unwrap(),
            from_hex("0x3ac884bddcf5d47b").unwrap(),
            from_hex("0xdc6cac43bad89d05").unwrap(),
            from_hex("0x9b815d1f02e9496d").unwrap(),
        ],
    ];
    pub static ref RESCUE_PRIME_GOLDILOCKS_8_PARAMS: Arc<RescuePrimeParams<Scalar>> = Arc::new(
        RescuePrimeParams::new(8, 7, [0x92492491b6db6db7, 0x0, 0x0, 0x0,], 8, &MDS8, &RC8)
    );
    pub static ref RESCUE_PRIME_GOLDILOCKS_12_PARAMS: Arc<RescuePrimeParams<Scalar>> =
        Arc::new(RescuePrimeParams::new(
            12,
            7,
            [0x92492491b6db6db7, 0x0, 0x0, 0x0,],
            8,
            &MDS12,
            &RC12
        ));
}
