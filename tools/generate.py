#!/usr/bin/env python3
'''
Code generation helper tool.
It helps me generate Rust and Assembly code by reading sbi.jsonc
'''
import json5 as json
import argparse

f = open('./sbi.jsonc', "r", encoding="utf-8")
extensions = json.load(f)
f.close()

c_to_rust = {
    'long': 'isize',
    'unsigned long': 'usize',
    'uint8_t': 'u8',
    'uint32_t': 'u32',
    'uint64_t': 'u64'
}


def print_extensions(extensions):
    print("supported extensions...")

    for extension in extensions:
        extension_name = extension.get("name")
        eid = extension.get("eid")
        print(f"{extension_name} (EID: {eid})")


def check(extensions):
    print("checking extensions...")

    for extension in extensions:
        print()

        extension_name = extension.get("name")
        functions = extension.get("functions")

        print(extension_name)
        for function in functions:
            function_name = function.get("name")
            fid = function.get("fid")

            print(f"{function_name:<30}: fid {fid:>2}")


def generate_assembly(extensions):
    for extension in extensions:
        eid = int(extension.get("eid"), 16)
        functions = extension.get("functions")

        for function in functions:
            name = function.get("name")
            fid = function.get("fid")

            print(f".globl {name}")
            print(f"{name}:")
            print(f"\tli a6, {fid}")
            print(f"\tli a7, {eid}")
            print(f"\tecall")
            print(f"\tret")
            print()


def count_types(extensions):
    frequency = dict()

    for extension in extensions:
        functions = extension.get("functions")
        for function in functions:
            args = function.get("args")
            if args:
                for arg in args:
                    arg_type = arg.get("type")
                    if arg_type in frequency:
                        frequency[arg_type] += 1
                    else:
                        frequency[arg_type] = 1

    for arg_type, count in frequency.items():
        print(f"{arg_type:<13} -> {c_to_rust.get(arg_type):<5} : {count:>2}")


def generate_rust(extensions):
    for extension in extensions:
        extension_name = extension.get("name")
        extension_name = extension_name.replace(
            " ", "_").removesuffix("_extension")

        print(f"pub mod {extension_name} {{")
        print("\tunsafe extern \"C\" {")
        functions = extension.get("functions")
        for function in functions:
            function_name = function.get("name")

            args = function.get("args")
            args_results = list()
            args_string = str()

            if args:
                for arg in args:
                    arg_name = arg.get("name")
                    arg_type = arg.get("type")
                    arg_result = f"{arg_name}: {c_to_rust.get(arg_type)}"
                    args_results.append(arg_result)

            if len(args_results) >= 1:
                args_string = ", ".join(args_results)

            print(f"\t\tpub fn {function_name}({args_string}) -> SbiRet;")

        print("\t}\n}\n")


parser = argparse.ArgumentParser(
    description='Generate RISC-V assembly from SBI extensions')
parser.add_argument('--print-extensions', action='store_true',
                    help='Print supported extensions')
parser.add_argument('--check', action='store_true', help='Check extensions')
parser.add_argument('--generate-assembly',
                    action='store_true', help='Generate assembly code')
parser.add_argument('--generate-rust',
                    action='store_true', help='Generate Rust code')
parser.add_argument('--count-types', action='store_true',
                    help='Count argument types')

args = parser.parse_args()

if args.print_extensions:
    print_extensions(extensions)
elif args.check:
    check(extensions)
elif args.count_types:
    count_types(extensions)
elif args.generate_assembly:
    generate_assembly(extensions)
elif args.generate_rust:
    generate_rust(extensions)
else:
    generate_assembly(extensions)
