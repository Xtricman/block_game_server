import shutil
import os
import array
import re
import sys

if len(sys.argv)==2 and sys.argv[1] == "build":
    os.mkdir("build/")
    shutil.copy("Cargo.toml", "build/")
    shutil.copytree("src", "build/src")

    a = array.array('H')

    with os.scandir("src/dynamic_types") as dm:
        for i in dm:
            if re.match(r"\A_[1-9][0-9]*\.rs\Z|\A_0\.rs\Z", i.name) != None:
                x = int(i.name[1:len(i.name)-3], 10)
                if x<65536:
                    a.append(x)
            elif not (i.name in ("_null.rs", "mod.rs")):
                os.remove("build/src/dynamic_types/"+i.name)


    mod_decl_list = "\n\npub mod _null;\n"
    mod_info_list = "\n"

    for i in range(0,max(a)+1):
        if i in a:
            mod_decl_list += "pub mod _{};\n".format(i)
            mod_info_list += "_{}::Module::ID_MODULE_INFO,\n".format(i)
        else:
            mod_info_list += "_null::Module::ID_MODULE_INFO,\n"
    
    feature_map_decl = "static FEATURE_MAP: [IDModuleInfo; {}] = [{}];\n".format(max(a)+1, mod_info_list)

    with open("build/src/dynamic_types/mod.rs", mode="at", encoding='UTF-8', newline='\n') as tf:
        print(mod_decl_list, file=tf)
        print(feature_map_decl, file=tf)

    os.chdir("build/src")
    os.system("cargo build")
elif len(sys.argv)==2 and sys.argv[1] == "clean":
    shutil.rmtree("build", ignore_errors=True)
else:
    print("argument must be clean or build, must run clean before every build")