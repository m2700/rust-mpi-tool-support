import patterns

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
