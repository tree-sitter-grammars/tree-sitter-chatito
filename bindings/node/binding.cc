#include "nan.h"
#include "tree_sitter/parser.h"
#include <node.h>

using namespace v8;

extern "C" TSLanguage * tree_sitter_chatito();
extern "C" TSLanguage * tree_sitter_chatl();
// extern "C" TSLanguage * tree_sitter_chatette();

namespace {

NAN_METHOD(New) {}

void Init(Local<Object> exports, Local<Object> module) {
  Local<FunctionTemplate> chatito_tpl = Nan::New<FunctionTemplate>(New);
  chatito_tpl->SetClassName(Nan::New("Language").ToLocalChecked());
  chatito_tpl->InstanceTemplate()->SetInternalFieldCount(1);
  Local<Function> chatito_constructor = Nan::GetFunction(chatito_tpl).ToLocalChecked();
  Local<Object> chatito_instance = chatito_constructor->NewInstance(Nan::GetCurrentContext()).ToLocalChecked();
  Nan::SetInternalFieldPointer(chatito_instance, 0, tree_sitter_chatito());
  Nan::Set(chatito_instance, Nan::New("name").ToLocalChecked(), Nan::New("chatito").ToLocalChecked());

  Local<FunctionTemplate> chatl_tpl = Nan::New<FunctionTemplate>(New);
  chatl_tpl->SetClassName(Nan::New("Language").ToLocalChecked());
  chatl_tpl->InstanceTemplate()->SetInternalFieldCount(1);
  Local<Function> chatl_constructor = Nan::GetFunction(chatl_tpl).ToLocalChecked();
  Local<Object> chatl_instance = chatl_constructor->NewInstance(Nan::GetCurrentContext()).ToLocalChecked();
  Nan::SetInternalFieldPointer(chatl_instance, 0, tree_sitter_chatl());
  Nan::Set(chatl_instance, Nan::New("name").ToLocalChecked(), Nan::New("chatl").ToLocalChecked());

  // Local<FunctionTemplate> chatette_tpl = Nan::New<FunctionTemplate>(New);
  // chatette_tpl->SetClassName(Nan::New("Language").ToLocalChecked());
  // chatette_tpl->InstanceTemplate()->SetInternalFieldCount(1);
  // Local<Function> chatette_constructor = Nan::GetFunction(chatette_tpl).ToLocalChecked();
  // Local<Object> chatette_instance = chatette_constructor->NewInstance(Nan::GetCurrentContext()).ToLocalChecked();
  // Nan::SetInternalFieldPointer(chatette_instance, 0, tree_sitter_chatette());
  // Nan::Set(chatette_instance, Nan::New("name").ToLocalChecked(), Nan::New("chatette").ToLocalChecked());

  Nan::Set(exports, Nan::New("chatito").ToLocalChecked(), chatito_instance);
  Nan::Set(exports, Nan::New("chatl").ToLocalChecked(), chatl_instance);
  // Nan::Set(exports, Nan::New("chatette").ToLocalChecked(), chatette_instance);
}

NODE_MODULE(tree_sitter_chatito_binding, Init)

}  // namespace
