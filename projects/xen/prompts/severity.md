# Xen Finding Severity

- Critical: demonstrated guest-to-hypervisor compromise or similarly broad
  loss of host isolation, reachable in a supported configuration.
- High: demonstrated cross-domain confidentiality/integrity failure, host
  crash, or privileged execution with clearly stated prerequisites.
- Medium: concrete functional failure, resource leak, or narrower availability
  or correctness regression under a reachable configuration.
- Low: concrete limited-impact defect without material isolation consequences.
- Unknown: a confirmed defect whose practical impact cannot be established.

Explain prerequisites, affected configurations, trust boundary and impact.
Never infer severity merely from a suspicious operation or subsystem name.