use ark_bn254::Bn254;
use ark_ec::pairing::Pairing;
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::kzg10::KZG10;
use ark_std::test_rng;

type UniPoly254 = DensePolynomial<<Bn254 as Pairing>::ScalarField>;

fn main() {
    let rng = &mut test_rng();

    // `params` is of type {unknown} and no delcaration is found for `setup`
    let params = KZG10::<Bn254, UniPoly254>::setup(10, false, rng).expect("Setup failed");
}
