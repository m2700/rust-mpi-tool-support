#!/usr/bin/env python3

import io, os
import sys
import patterns
from utility import *

class Eof(Exception):
    pass

class ExternFunction:
    @classmethod
    def parse(cls, input_file):
        tk_extern = parse_token(input_file)
        if tk_extern == "":
            raise Eof()
        assert tk_extern == "extern"
        assert parse_token(input_file) == '"C"'
        funcion_scope_tokens = parse_token_group(input_file)
        assert funcion_scope_tokens[:3] == ["{", "pub", "fn"]
        assert funcion_scope_tokens[-2:] == [";", "}"]

        function_ident = funcion_scope_tokens[3]
        arg_ret_line = funcion_scope_tokens[4:-2]
        return cls(function_ident, arg_ret_line)

    def __init__(self, function_ident, arg_ret_line):
        self.function_ident = function_ident
        self.arg_ret_line = arg_ret_line

    def format_qmpi_macro(self):
        assert self.function_ident[:4] == "MPI_"
        mpiless_func_ident = self.function_ident[4:]
        intrcpt_func_ident = "E_" + mpiless_func_ident

        mcr_arg_ret_line = \
            tokens_to_string(self.arg_ret_line, patterns.macro_type_map)

        return \
            f"fn {intrcpt_func_ident}\
[{patterns.macro_mpi_func_id_prefix}{mpiless_func_ident}, \
{mpiless_func_ident.lower()}]\
{mcr_arg_ret_line};"

    def format_pmpi_macro(self):
        assert self.function_ident[:4] == "MPI_"
        mpiless_func_ident = self.function_ident[4:]

        mcr_arg_ret_line = \
            tokens_to_string(self.arg_ret_line, patterns.macro_type_map)

        return \
            f"fn {self.function_ident}\
[{mpiless_func_ident.lower()}]\
{mcr_arg_ret_line};"

    def format_trait(self):
        assert self.function_ident[:4] == "MPI_"
        mpiless_func_ident = self.function_ident[4:]

        mcr_arg_ret_line = \
            tokens_to_string(self.arg_ret_line, patterns.type_map)
        assert mcr_arg_ret_line[0] == "("

        mcr_arg_ret_line_reader = io.StringIO(mcr_arg_ret_line)
        arg_line = parse_token_group(mcr_arg_ret_line_reader)
        assert arg_line[0] == "("
        assert arg_line[-1] == ")"

        assert parse_token(mcr_arg_ret_line_reader) == "->"
        return_type_tkns = []
        tkn = parse_token(mcr_arg_ret_line_reader)
        while tkn != "":
            return_type_tkns.append(tkn)
            tkn = parse_token(mcr_arg_ret_line_reader)
        return_type = tokens_to_string(return_type_tkns, {})

        arg_type_tokens = list(iter_split(arg_line[1:-1], ","))
        if arg_type_tokens != [] and arg_type_tokens[-1] == []:
            arg_type_tokens = arg_type_tokens[:-1]
        if arg_type_tokens != [] and arg_type_tokens[-1] == ["..."]:
            arg_type_tokens = arg_type_tokens[:-1]

        arg_id_list = []
        type_tkns_list = []
        for arg_type_tkns in arg_type_tokens:
            assert arg_type_tkns[1] == ":"
            arg_id_list.append(arg_type_tkns[0])
            type_tkns_list.append(
                tokens_to_string(arg_type_tkns[2:], {})
            )

        types_joined = ", ".join(type_tkns_list)
        arg_ids_joined = ",".join(arg_id_list)
        mcr_arg_ret_line_cont = mcr_arg_ret_line[1:]

        return \
            f"#[inline] fn {mpiless_func_ident.lower()}\
<F>(next_f: UnsafeBox<F>, \
{mcr_arg_ret_line_cont} \
where F: FnOnce({types_joined}) -> {return_type} \
{{ unsafe{{ next_f.unwrap()({arg_ids_joined}) }} }}"

    def format_fn_counter(self):
        assert self.function_ident[:4] == "MPI_"
        mpiless_func_ident = self.function_ident[4:]

        mcr_arg_ret_line = \
            tokens_to_string(self.arg_ret_line, patterns.type_map)
        assert mcr_arg_ret_line[0] == "("

        mcr_arg_ret_line_reader = io.StringIO(mcr_arg_ret_line)
        arg_line = parse_token_group(mcr_arg_ret_line_reader)
        assert arg_line[0] == "("
        assert arg_line[-1] == ")"

        assert parse_token(mcr_arg_ret_line_reader) == "->"
        return_type_tkns = []
        tkn = parse_token(mcr_arg_ret_line_reader)
        while tkn != "":
            return_type_tkns.append(tkn)
            tkn = parse_token(mcr_arg_ret_line_reader)
        return_type = tokens_to_string(return_type_tkns, {})

        arg_type_tokens = list(iter_split(arg_line[1:-1], ","))
        if arg_type_tokens != [] and arg_type_tokens[-1] == []:
            arg_type_tokens = arg_type_tokens[:-1]
        if arg_type_tokens != [] and arg_type_tokens[-1] == ["..."]:
            arg_type_tokens = arg_type_tokens[:-1]

        arg_id_list = []
        type_tkns_list = []
        for arg_type_tkns in arg_type_tokens:
            assert arg_type_tkns[1] == ":"
            arg_id_list.append(arg_type_tkns[0])
            type_tkns_list.append(
                tokens_to_string(arg_type_tkns[2:], {})
            )

        types_joined = ", ".join(type_tkns_list)
        arg_ids_joined = ",".join(arg_id_list)
        mcr_arg_ret_line_cont = mcr_arg_ret_line[1:]

        return \
            f"#[inline] fn {mpiless_func_ident.lower()}\
<F>(next_f: UnsafeBox<F>, \
{mcr_arg_ret_line_cont} \
where F: FnOnce({types_joined}) -> {return_type} \
{{\
    *mpi_fn_counter_map.lock().unwrap().entry(\"{mpiless_func_ident}\").or_insert(0) += 1;\
    unsafe{{next_f.unwrap()({arg_ids_joined})\
}} }}"

input_file=open("input.txt")
qmpi_macro_output=open("qmpi_macro_output.txt", "w")
trait_functions_output=open("trait_functions_output.txt", "w")
pmpi_macro_output=open("pmpi_macro_output.txt", "w")
fn_counter_output=open("fn_counter_output.txt", "w")

while True:
    try:
        func = ExternFunction.parse(input_file)
    except Eof:
        break
    
    qmpi_macro_output.write(func.format_qmpi_macro() + "\n")
    pmpi_macro_output.write(func.format_pmpi_macro() + "\n")
    trait_functions_output.write(func.format_trait() + "\n")
    fn_counter_output.write(func.format_fn_counter() + "\n")
