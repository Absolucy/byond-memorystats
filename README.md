# byond-memorystats

This is a simple library to programatically get the memory stats from DreamDaemon for a running BYOND server.

Normally, you can only get this string through the DreamDaemon UI on Windows, sending the SIGUSR2 signal on Linux, or the "interactive console", but this allows you to get the same string at runtime, by calling the external library.

Pre-built binaries are available through the "Build Binaries" github action runs.

## Usage

Usage is simple:
```dm
// memorystats.dll on Windows, libmemorystats.so on Linux
var/stats = call_ext(world.system_type == MS_WINDOWS ? "./memorystats.dll" : "./libmemorystats.so", "memory_stats")()
```

It will return a string in this format:

```
Server mem usage:
prototypes:
	obj: 3.32 KB (20)
	mob: 16 B (1)
	proc: 10.7 KB (69)
	str: 25.3 KB (635)
	appearance: 0 B (0)
	filter: 0 B (0)
	id array: 13.6 KB (147)
	map: 8 KB (0,0,0)
objects:
	mobs: 0 B (0)
	objs: 0 B (0)
	datums: 0 B (0)
	images: 0 B (0)
	lists: 56 B (2)
	procs: 688 B (2)
```

## License

byond-memorystats is licensed under the [Mozilla Public License Version 2.0 (MPL-2.0)](LICENSE.md)
