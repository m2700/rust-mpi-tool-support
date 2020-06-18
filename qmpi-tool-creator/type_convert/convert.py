#!/usr/bin/env python3

import patterns
from utility import *

with open("input.txt") as input_file:
    with open("type_convert_output.txt", "w") as type_convert_output_file:
        with open("trait_functions_output.txt", "w") as trait_functions_output_file:
            for line in input_file.readlines():
                macro_arg_line = []
                rust_arg_line = []
                rust_arg_types = []
                rust_arg_idents = []
                line = line.strip()
                if line.startswith("fn ") and line.endswith(";"):
                    line = line[3:-1].strip()

                    line, fn_ident = cut_of_ident_front(line)
                    assert len(fn_ident) > 0
                    line, res_tp = cut_of_ident_back(line)
                    assert len(res_tp) > 0
                    rust_res_tp = patterns.type_map.get(res_tp);
                    if rust_res_tp == None:
                        print("no replacement for \"", res_tp, "\"", sep = "")
                        exit(-1)
                    macro_rust_res_tp = patterns.macro_type_map.get(res_tp);
                    if macro_rust_res_tp == None:
                        print("no macro replacement for \"", res_tp, "\"", sep = "")
                        exit(-1)
                    line = line.strip()

                    assert line.endswith("->")
                    line = line[:-2].strip()

                    assert line.startswith("(")
                    assert line.endswith(")")
                    line = line[1:-1].strip()

                    if line != "":
                        for arg in line.split(","):
                            pre_arg, arg_name, post_arg = extract_arg_name(arg)
                            pre_arg, post_arg = pre_arg.strip(), post_arg.strip()
                            arg_name = into_valid_rust_ident(arg_name)
                            assert len(pre_arg) > 0

                            macro_repl_type = patterns.macro_type_replacements.get((pre_arg, post_arg))
                            if macro_repl_type == None:
                                print("no macro replacement for \"", pre_arg,
                                      "\" <arg_name> \"", post_arg, "\"", sep = "")
                                exit(-1)
                            macro_arg_line.append(arg_name + ": " + macro_repl_type)

                            repl_type = patterns.type_replacements.get((pre_arg, post_arg))
                            if repl_type == None:
                                print("no replacement for \"", pre_arg,
                                      "\" <arg_name> \"", post_arg, "\"", sep = "")
                                exit(-1)
                            rust_arg_line.append(arg_name + ": " + repl_type)
                            rust_arg_types.append(repl_type)
                            rust_arg_idents.append(arg_name)

                    assert fn_ident.startswith("E_")
                    intercept_ident = fn_ident[2:].lower()

                    type_convert_output_file.write(
                        "fn " + fn_ident + "[" + patterns.macro_mpi_func_id_prefix + fn_ident[2:] + ", "
                         + intercept_ident
                         + "](" + ", ".join(macro_arg_line) + ") -> "
                         + macro_rust_res_tp + ";\n"
                    )

                    trait_functions_output_file.write(
                        "#[inline]fn " + intercept_ident + "<F>(&self, next_f: F, "
                         + ", ".join(rust_arg_line) + ") -> " + rust_res_tp
                         + " where F: FnOnce(" + ", ".join(rust_arg_types)
                         + ") -> " + rust_res_tp + " {next_f(" + ",".join(rust_arg_idents) + ")}\n"
                    )

                else:
                    print("unexpected line:", repr(line))
                    exit(-1)
