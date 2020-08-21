use super::ptr_j::*;
use super::result::ToJniResult;
use super::string::ToJniString;
use super::string::ToString;
use crate::panic::{handle_exception_result, ToResult, Zip};
use crate::ptr::RPtrRepresentable;
use jni::objects::{JObject, JString};
use jni::sys::{jbyteArray, jlong, jobject};
use jni::JNIEnv;
use cardano_serialization_lib::crypto::{Bip32PublicKey};
use std::convert::TryFrom;


#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_io_emurgo_rnhaskellshelley_Native_bip32PublicKeyToRawKey(
  env: JNIEnv, _: JObject, bip_32_public_key: JRPtr
) -> jobject {
  handle_exception_result(|| {
    let bip_32_public_key = bip_32_public_key.rptr(&env)?;
    bip_32_public_key
      .typed_ref::<Bip32PublicKey>()
      .map(|bip_32_public_key| bip_32_public_key.to_raw_key())
      .and_then(|raw_public_key| raw_public_key.rptr().jptr(&env))
  })
  .jresult(&env)
}