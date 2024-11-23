#include <napi.h>

typedef struct TSLanguage TSLanguage;

extern "C" TSLanguage *tree_sitter_chatito();
extern "C" TSLanguage *tree_sitter_chatl();

// "tree-sitter", "language" hashed with BLAKE2
const napi_type_tag LANGUAGE_TYPE_TAG = {
    0x8AF2E5212AD58ABF, 0xD5006CAD83ABBA16
};

Napi::Object Init(Napi::Env env, Napi::Object exports) {
    auto chatito = Napi::Object::New(env);
    chatito["name"] = Napi::String::New(env, "chatito");
    auto chatito_language = Napi::External<TSLanguage>::New(env, tree_sitter_chatito());
    chatito_language.TypeTag(&LANGUAGE_TYPE_TAG);
    chatito["language"] = chatito_language;

    auto chatl = Napi::Object::New(env);
    chatl["name"] = Napi::String::New(env, "chatl");
    auto chatl_language = Napi::External<TSLanguage>::New(env, tree_sitter_chatl());
    chatl_language.TypeTag(&LANGUAGE_TYPE_TAG);
    chatl["language"] = chatl_language;

    exports["chatito"] = chatito;
    exports["chatl"] = chatl;
    return exports;
}

NODE_API_MODULE(tree_sitter_chatito_binding, Init)
