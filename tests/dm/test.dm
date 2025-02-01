#define DEFAULT_DLL_PATH (world.system_type == MS_WINDOWS ? "./memorystats.dll" : "./libmemorystats.so")

#if DM_VERSION >= 515
#define LIBCALL call_ext
#else
#define LIBCALL call
#endif

/world
	visibility = FALSE

/world/New()
	var/lib
	if("lib" in params)
		lib = params["lib"]
	else
		lib = DEFAULT_DLL_PATH
	world.log << "lib path: [lib]"
	try
		run_tests(lib)
		world.log << "success"
	catch(var/exception/err)
		world.log << "::error::[err]"
		world.log << "fail"
	del(src)


/proc/run_tests(lib)
	var/output = LIBCALL(lib, "memory_stats")()
	if(!output)
		CRASH("memory_stats output was blank")
	world.log << "[output]"
	if(findtext(output, "error: "))
		CRASH("memory_stats errored: [copytext(output, 8)]")
	if(!findtext(output, "Server mem usage"))
		CRASH("Expected string not found: \"Server mem usage\"")
	if(!findtext(output, "prototypes"))
		CRASH("Expected string not found: \"prototypes\"")
	if(!findtext(output, "objects"))
		CRASH("Expected string not found: \"objects\"")
	return TRUE
