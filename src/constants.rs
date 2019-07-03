//! Contains the curve-constants needed by the different algorithm implementations.

use crate::scalar::Scalar;
use crate::field::FieldElement;

/// Edwards `a` variable value = `-1 (mod l)` equals:
/// `7237005577332262213973186563042994240857116359379907606001950938285454250988`
/// where `l = Prime of the field = 2^252 + 27742317777372353535851937790883648493`
pub static EDWARDS_A: FieldElement = FieldElement([671914833335276, 3916664325105025, 1367801, 0, 17592186044416]);

/// Edwards `d` variable value = `86649/86650 (mod l)` equals:
/// `1201935917638644956968126114584555454358623906841733991436515590915937358637`
/// where `l = Prime of the field = 2^252 + 27742317777372353535851937790883648493`
pub static EDWARDS_D: FieldElement = FieldElement([939392471225133, 587442007554368, 4497154776428662, 4184267646867733, 2921744366591]);
