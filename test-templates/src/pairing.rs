#[macro_export]
macro_rules! test_pairing {
    ($mod_name: ident; $Pairing: ty) => {
        mod $mod_name {
            pub const ITERATIONS: usize = 100;
            use ark_ec::{pairing::*, CurveGroup, Group};
            use ark_ff::{Field, PrimeField};
            use ark_std::{test_rng, One, UniformRand, Zero};
            #[test]
            fn test_bilinearity_projective() {
                for _ in 0..100 {
                    let mut rng = test_rng();
                    let a: <$Pairing as Pairing>::G1 = UniformRand::rand(&mut rng);
                    let b: <$Pairing as Pairing>::G2 = UniformRand::rand(&mut rng);
                    let s: <$Pairing as Pairing>::ScalarField = UniformRand::rand(&mut rng);

                    let sa = a * s;
                    let sb = b * s;

                    let ans1 = <$Pairing>::pairing(sa, b);
                    let ans2 = <$Pairing>::pairing(a, sb);
                    let ans3 = <$Pairing>::pairing(a, b) * s;

                    assert_eq!(ans1, ans2);
                    assert_eq!(ans2, ans3);

                    assert_ne!(ans1, PairingOutput::zero());
                    assert_ne!(ans2, PairingOutput::zero());
                    assert_ne!(ans3, PairingOutput::zero());
                    let group_order = <<$Pairing as Pairing>::ScalarField>::characteristic();

                    assert_eq!(ans1.mul_bigint(group_order), PairingOutput::zero());
                    assert_eq!(ans2.mul_bigint(group_order), PairingOutput::zero());
                    assert_eq!(ans3.mul_bigint(group_order), PairingOutput::zero());
                }
            }

            #[test]
            fn test_bilinearity_affine() {
                for _ in 0..100 {
                    let mut rng = test_rng();
                    let a: <$Pairing as Pairing>::G1 = UniformRand::rand(&mut rng);
                    let b: <$Pairing as Pairing>::G2 = UniformRand::rand(&mut rng);
                    let s: <$Pairing as Pairing>::ScalarField = UniformRand::rand(&mut rng);

                    let sa = a * s;
                    let sb = b * s;

                    let ans1 = <$Pairing>::pairing_affine(sa, b.into_affine());
                    let ans2 = <$Pairing>::pairing_affine(a, sb.into_affine());
                    let ans3 = <$Pairing>::pairing_affine(a, b.into_affine()) * s;

                    assert_eq!(ans1, ans2);
                    assert_eq!(ans2, ans3);

                    assert_ne!(ans1, PairingOutput::zero());
                    assert_ne!(ans2, PairingOutput::zero());
                    assert_ne!(ans3, PairingOutput::zero());
                    let group_order = <<$Pairing as Pairing>::ScalarField>::characteristic();

                    assert_eq!(ans1.mul_bigint(group_order), PairingOutput::zero());
                    assert_eq!(ans2.mul_bigint(group_order), PairingOutput::zero());
                    assert_eq!(ans3.mul_bigint(group_order), PairingOutput::zero());
                }
            }

            #[test]
            fn test_multi_pairing_projective() {
                for _ in 0..ITERATIONS {
                    let rng = &mut test_rng();

                    let a = <$Pairing as Pairing>::G1::rand(rng);
                    let b = <$Pairing as Pairing>::G2::rand(rng);
                    let c = <$Pairing as Pairing>::G1::rand(rng);
                    let d = <$Pairing as Pairing>::G2::rand(rng);
                    let ans1 = <$Pairing>::pairing(a, b) + &<$Pairing>::pairing(c, d);
                    let ans2 = <$Pairing>::multi_pairing(&[a, c], &[b, d]);
                    assert_eq!(ans1, ans2);
                }
            }

            #[test]
            fn test_multi_pairing_affine() {
                for _ in 0..ITERATIONS {
                    let rng = &mut test_rng();

                    let a = <$Pairing as Pairing>::G1::rand(rng).into_affine();
                    let b = <$Pairing as Pairing>::G2::rand(rng).into_affine();
                    let c = <$Pairing as Pairing>::G1::rand(rng).into_affine();
                    let d = <$Pairing as Pairing>::G2::rand(rng).into_affine();
                    let ans1 = <$Pairing>::pairing_affine(a, b) + &<$Pairing>::pairing_affine(c, d);
                    let ans2 = <$Pairing>::multi_pairing_affine(&[a, c], &[b, d]);
                    assert_eq!(ans1, ans2);
                }
            }

            #[test]
            fn test_pairing_affine_vs_projective() {
                let rng = &mut test_rng();

                let a_proj = <$Pairing as Pairing>::G1::rand(rng);
                let b_proj = <$Pairing as Pairing>::G2::rand(rng);
                let a_affine = a_proj.into_affine();
                let b_affine = b_proj.into_affine();

                let ans1 = <$Pairing>::multi_pairing(&[a_proj], &[b_proj]);
                let ans2 = <$Pairing>::multi_pairing_affine(&[a_affine], &[b_affine]);
                assert_eq!(ans1, ans2);
            }

            #[test]
            fn test_final_exp() {
                for _ in 0..ITERATIONS {
                    let rng = &mut test_rng();
                    let fp_ext = <$Pairing as Pairing>::TargetField::rand(rng);
                    let gt = <$Pairing as Pairing>::final_exponentiation(MillerLoopOutput(fp_ext))
                        .unwrap()
                        .0;
                    let r = <$Pairing as Pairing>::ScalarField::MODULUS;
                    assert!(gt.cyclotomic_exp(r).is_one());
                }
            }
        }
    };
}
