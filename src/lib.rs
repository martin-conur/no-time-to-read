#![allow(unused)]
#![allow(dead_code)]

#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

#[cfg(feature = "accelerate")]
extern crate accelerate_src;

use std::str::FromStr;

use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, HiddenAct, DTYPE};
use clap::ValueEnum;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::{PaddingParams, Tokenizer};

#[derive(Clone, Copy, ValueEnum, Debug)]
pub enum Language {
    Spanish,
    English,
}

pub struct SummarizeModel<'a> {
    text: &'a str,
    language: Language,
    cpu: bool,
    tracing: bool,
    aproximate_gelu: bool,
}

impl SummarizeModel<'_> {
    fn build_model_and_tokenizer(&self) -> Result<(BertModel, Tokenizer)> {
        // setting the device, CPU or CUDA
        // TODO: the hability to choose
        let device = candle_core::Device::cuda_if_available(0)?;
        // Selecting the model based on the language
        // could be a multiliungual model in the future*
        let (model_id, revision) = match self.language {
            Language::Spanish => (
                "mrm8488/bert2bert_shared-spanish-finetuned-summarization".to_string(),
                "main".to_string(),
            ),
            Language::English => todo!(), // just Spanish for now
        };
        // get the repository
        let repo = Repo::with_revision(model_id, RepoType::Model, revision);

        // downloading the files to build our model and tokenizer
        let (config_filename, tokenizer_filename, weights_filename) = {
            let api = Api::new()?;
            let api = api.repo(repo);
            let config = api.get("config.json")?;
            let tokenizer = api.get("tokenizer_config.json")?;
            // because our model has a safetensor format, we retreive its weights
            // from that file, if not, use the "pytorch_model.bin" file instead
            let weights = api.get("model.safetensors")?;
            (config, tokenizer, weights)
        };

        // creating the model and tokenizer based on the donwloaded files
        let config = std::fs::read_to_string(config_filename)?;
        let mut config: Config = serde_json::from_str(&config)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;
        // because our model has a safetensor format, create the model with 'from__mmaped_safetensors'
        // if not, use the 'VarBuilder::from_pth(&weights_filename, DTYPE, &device)?' instead
        let vb =
            unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)? };
        // if gelu aproximation asked
        if self.aproximate_gelu {
            config.hidden_act = HiddenAct::GeluApproximate;
        }
        let model = BertModel::load(vb, &config)?;
        Ok((model, tokenizer))
    }
}
