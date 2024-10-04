export type Process = {
  pid: string;
  cmd: string;
  process_name: string;
  pss_in_kb: number;
  rss_in_kb: number;
  uss_in_kb: number;
  time_userspace_miliseconds: number;
  time_kernel_miliseconds: number;
  start_time: number;
  cpu_usage_percentage: number;
};
