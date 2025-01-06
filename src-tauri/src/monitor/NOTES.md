# Process Summary

## Memory

- [x] Obtain memory from /proc/PID/smaps_rollup Pss (Proportional Set Size)
    Proportional part of memory pages shares with other processes
- [x] Rss (Resident Set Size) /proc/PID/smaps_rollup
- [x] Pid /proc/PID/stat 1st field (obtained from folder name)
- [x] process name /proc/PID/stat 2nd field
- [x] cmdline /proc/PID/cmdline
- [x] read count of fd open

- [x] Use Pss, Uss, Rss https://github.com/kwkroeger/smem/blob/master/smem for more details

- [x] Calculate CPU Usage https://www.baeldung.com/linux/total-process-cpu-usage
  - [x] /proc/uptime
    1st time in seconds since the system started
  - [x]/proc/pid/stat
    - utime 14th time of process in user mode
    - stime 15th time of process in kernel mode
    - 22nd start time
  - [x] Calc
        let PROCESS_ELAPSED_SEC="$SYSTEM_UPTIME_SEC - $PROCESS_STARTTIME_SEC"
        let PROCESS_USAGE_SEC="$PROCESS_UTIME_SEC + $PROCESS_STIME_SEC"
        let PROCESS_USAGE="$PROCESS_USAGE_SEC * 100 / $PROCESS_ELAPSED_SEC"

  - [x] getconf CLK_TCK to obtain clicks and transform stat times to seconds
- [x] Parallel https://docs.rs/rayon/latest/rayon/

- Processes
  - [x] Record scraping time
  - [ ] Accumulate up to 60 records every 5s
  - [x] Calculate diff with the last record
  - [x] Store Process in Hashmap

- System
  - [ ] Get CPU Usage from /proc/stat, time spend on different kinds of work on clicks
      1st user: normal processes executing in user mode
      3rd system: processes executing in kernel mode
      4th idle
  - [ ] Get Mem Usage from /proc/meminfo
      MemTotal:       15667100 kB - System memory
      MemAvailable:    3762864 kB - Memory available before having to swap
  - [ ] Accumulate up to 60 records every 5s

- Styling
 - https://getbootstrap.com/docs/5.3/components/navs-tabs/
 - Gnome System Monitor https://www.google.com/search?sca_esv=00efb85e6f8ba711&sxsrf=ADLYWIJt2slCChT7v3p23-qyCsihQzIvXQ:1716454289057&q=system+monitor&uds=ADvngMgB_2oBFo8AreHDKVw-lNLWaFlzoge-zM-ViyheC_aJJ4j1Gxo8ZWzaUXe9hyclgS6oaqzQO5icfNDFW3XV2bQlwlVongAjmdsVz4G2VTTdUQWtZLvsx3tM3Ee8fLwflTJSsmBgQSl_2FmchuoG5wLSoec-OiMyv0r146ofKLRRAV8tXQZQW-_hwg3iKY25koIaMiTd-IUnffHD8aGRd2104jQZXCr1QwWPdPHzhsVx5JjXg3p3c9dPrO_0Ofsjll2r33pdheVmVaoCV0Azz9r_BTXpWvxTmDKvb5uFRsNyYdyHHneYDxt0UbK0nLctvx0wZuVU&udm=2&prmd=ivnmbt&sa=X&ved=2ahUKEwiYirDBsqOGAxX-2wIHHSH6AQgQtKgLegQIDhAB&biw=1760&bih=838&dpr=1.09#vhid=pkL0-KJCGhzr5M&vssid=mosaic
