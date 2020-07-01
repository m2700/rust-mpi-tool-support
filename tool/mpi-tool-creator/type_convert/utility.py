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

def parse_token(file):
    s = ""
    spos = file.tell()
    c = file.read(1)
    while c != "":
        # print(c)
        if s == "" or \
           (s + c).strip() == "" or \
           (s + c).isalnum() or \
           (s + c).isidentifier() or \
           (s.isdecimal() and c == ".") or \
           all(map(str.isdecimal, s.split(".", maxsplit=1))) or \
           (s + c) == "..":
            s += c
        elif (s + c).startswith('"'):
            s += c
            if s != "" and c == '"':
                break
        elif (s + c) in {"::", "->", "..."}:
            s += c
            break
        elif s != "" and c != c.strip():
            break
        else:
            file.seek(spos)
            break
        s = s.strip()

        spos = file.tell()
        c = file.read(1)

    return s.strip()

# import io
# r = io.StringIO("")
# print("parsed:", repr(parse_token(r)))
# print("parsed:", repr(parse_token(r)))
# print("parsed:", repr(parse_token(r)))
# assert False

klammer_map = {
    "{": "}",
    "[": "]",
    "(": ")"
}

def parse_token_group(file):
    first_token = parse_token(file)
    tokens = [first_token]
    if first_token in {"{", "[", "("}:
        kl_counter = 1
        while kl_counter > 0:
            tkn = parse_token(file)
            if tkn == first_token:
                kl_counter += 1
            elif tkn == klammer_map[first_token]:
                kl_counter -= 1
            tokens.append(tkn)
    return tokens

def tokens_to_string(tokens, type_map):
    s = ""
    for tkn in tokens:
        if not ((s.endswith("::") and tkn != ":") or \
                (tkn == "::" and \
                 not s.endswith("const") and \
                 not s.endswith("mut") and \
                 not s.endswith("->") and \
                 not s.endswith(":"))) \
           and not s.endswith("$") \
           and s != "":
            s += " "
        if tkn in type_map.keys():
            s += type_map[tkn]
        else:
            s += tkn
    return s

def iter_split(l, spl):
    spl_itm = []
    for itm in l:
        if itm == spl:
            yield spl_itm
            spl_itm = []
        else:
            spl_itm.append(itm)
    yield spl_itm
