#!/usr/bin/env python3

import patterns;

def block_strip(s, blocks):
    new_s = ""
    while new_s != s:
        new_s = s
        for blck in blocks:
            if s.startswith(blck):
                s = s[len(blck):]
            if s.endswith(blck):
                s = s[:-len(blck)]
    return s

def cut_of_ident_front(s):
    ctidx = 0
    while s[:(ctidx+1)].isidentifier():
        ctidx += 1
    return (s[ctidx:], s[:ctidx])

def cut_of_ident_back(s):
    ctidx = 0
    while s[-(ctidx+1):].isidentifier() or s[-(ctidx+1):].isdecimal():
        ctidx += 1
    return s[:-ctidx], s[-ctidx:]

def cut_of_number_front(s):
    ctidx = 0
    while s[:(ctidx+1)].isdecimal():
        ctidx += 1
    return s[ctidx:], s[:ctidx]

def extract_arg_name(arg):
    arg = arg.strip()

    arg_name, type_name = cut_of_ident_front(arg)
    assert len(type_name) > 0
    if type_name == "const":
        arg_name, type_name = cut_of_ident_front(arg_name.strip())
        assert len(type_name) > 0

    def strip_type_info(arg_name):
        return block_strip(arg_name.strip(),
                           {"*", "[]", "[3]", "const"})

    new_arg_name = strip_type_info(arg_name)
    while new_arg_name != arg_name:
        arg_name = new_arg_name
        new_arg_name = strip_type_info(arg_name)

    assert arg_name.isidentifier()

    arg_spl = arg.split(arg_name)
    pre_arg, post_arg = arg_name.join(arg_spl[:-1]), arg_spl[-1]

    return pre_arg, arg_name, post_arg

def into_valid_rust_ident(ident):
    if ident in patterns.rust_keywords:
        ident = "r#" + ident
    return ident

with open("input.txt") as input_file:
    with  open("output.txt", "w") as output_file:
        for line in input_file.readlines():
            new_arg_line = []
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

                        repl_type = patterns.type_replacements.get((pre_arg, post_arg))
                        if repl_type == None:
                            print("no replacement for \"", pre_arg,
                                  "\" <arg_name> \"", post_arg, "\"", sep = "")
                            exit(-1)
                        new_arg_line.append(arg_name + ": " + repl_type)

                assert fn_ident.startswith("E_")
                output_file.write(
                    "fn " + fn_ident + "[" + patterns.mpi_func_id_prefix + fn_ident[2:]
                     + "](" + ", ".join(new_arg_line) + ") -> "
                     + rust_res_tp + ";\n")

            else:
                print("unexpected line:", repr(line))
                exit(-1)
