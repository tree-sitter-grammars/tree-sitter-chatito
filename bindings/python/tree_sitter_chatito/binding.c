#include <Python.h>

typedef struct TSLanguage TSLanguage;

TSLanguage *tree_sitter_chatito(void);
TSLanguage *tree_sitter_chatl(void);

static PyObject* _binding_language_chatito(PyObject *Py_UNUSED(self), PyObject *Py_UNUSED(args)) {
    return PyCapsule_New(tree_sitter_chatito(), "tree_sitter.Language", NULL);
}

static PyObject* _binding_language_chatl(PyObject *Py_UNUSED(self), PyObject *Py_UNUSED(args)) {
    return PyCapsule_New(tree_sitter_chatl(), "tree_sitter.Language", NULL);
}

static PyMethodDef methods[] = {
    {"language_chatito", _binding_language_chatito, METH_NOARGS,
     "Get the tree-sitter language for the chatito grammar."},
    {"language_chatl", _binding_language_chatl, METH_NOARGS,
     "Get the tree-sitter language for the chatl grammar."},
    {NULL, NULL, 0, NULL}
};

static struct PyModuleDef module = {
    .m_base = PyModuleDef_HEAD_INIT,
    .m_name = "_binding",
    .m_doc = NULL,
    .m_size = -1,
    .m_methods = methods
};

PyMODINIT_FUNC PyInit__binding(void) {
    return PyModule_Create(&module);
}
