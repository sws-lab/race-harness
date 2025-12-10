// This is a stub skeleton for Linux kernel module verification.
// The skeleton contains variable and function declarations external to the module.
// Please fill-in appropriate function code and variable values to obtain complete definitions
// for verifier to work on.
//
// Make sure that your stub file compiles correctly in the kernel build directory. Compiler command line and
// include list are preliminary, so feel free to edit the stub as needed.
//
// When added to stub_generator script as an input, complete stub along with respective kernel modules shall
// produce an empty skeleton.
//
// CLANG COMMAND LINE: -I. -Wp,-MMD,./..module-common.o.d -nostdinc -I./arch/x86/include -I./arch/x86/include/generated -I./include -I./include -I./arch/x86/include/uapi -I./arch/x86/include/generated/uapi -I./include/uapi -I./include/generated/uapi -include ./include/linux/compiler-version.h -include ./include/linux/kconfig.h -include ./include/linux/compiler_types.h -D__KERNEL__ --target=x86_64-linux-gnu -fintegrated-as -Werror=unknown-warning-option -Werror=ignored-optimization-argument -Werror=option-ignored -Werror=unused-command-line-argument -Werror -std=gnu11 -fshort-wchar -funsigned-char -fno-common -fno-PIE -fno-strict-aliasing -mno-sse -mno-mmx -mno-sse2 -mno-3dnow -mno-avx -fcf-protection=branch -fno-jump-tables -m64 -falign-loops=1 -mno-80387 -mno-fp-ret-in-387 -mstack-alignment=8 -mskip-rax-setup -mtune=generic -mno-red-zone -mcmodel=kernel -Wno-sign-compare -fno-asynchronous-unwind-tables -mretpoline-external-thunk -mindirect-branch-cs-prefix -mfunction-return=thunk-extern -mharden-sls=all -fpatchable-function-entry=59,59 -fno-delete-null-pointer-checks -O2 -fstack-protector-strong -ftrivial-auto-var-init=pattern -fno-stack-clash-protection -fzero-call-used-regs=used-gpr -pg -mfentry -DCC_USING_NOP_MCOUNT -DCC_USING_FENTRY -falign-functions=64 -fstrict-flex-arrays=3 -fno-strict-overflow -fno-stack-check -Wall -Wundef -Werror=implicit-function-declaration -Werror=implicit-int -Werror=return-type -Werror=strict-prototypes -Wno-format-security -Wno-trigraphs -Wno-frame-address -Wno-address-of-packed-member -Wmissing-declarations -Wmissing-prototypes -Wframe-larger-than=2048 -Wno-gnu -Wno-format-overflow-non-kprintf -Wno-format-truncation-non-kprintf -Wvla -Wno-pointer-sign -Wcast-function-type -Wimplicit-fallthrough -Werror=date-time -Werror=incompatible-pointer-types -Wenum-conversion -Wextra -Wunused -Wno-unused-but-set-variable -Wno-unused-const-variable -Wno-format-overflow -Wno-override-init -Wno-pointer-to-enum-cast -Wno-tautological-constant-out-of-range-compare -Wno-unaligned-access -Wno-enum-compare-conditional -Wno-missing-field-initializers -Wno-type-limits -Wno-shift-negative-value -Wno-enum-enum-conversion -Wno-sign-compare -Wno-unused-parameter -DRANDSTRUCT -frandomize-layout-seed-file=./scripts/basic/randstruct.seed -fsanitize=array-bounds -fsanitize=shift -fsanitize=bool -fsanitize=enum -fsanitize-coverage=trace-pc -fsanitize-coverage=trace-cmp -fsanitize=thread -fno-optimize-sibling-calls -mllvm -tsan-compound-read-before-write=1 -mllvm -tsan-distinguish-volatile=1 -fdebug-info-for-profiling -mllvm -enable-fs-discriminator=true -mllvm -improved-fs-discriminator=true -gmlt -fbasic-block-sections=labels -DMODULE '-DKBUILD_BASENAME=".module_common"' '-DKBUILD_MODNAME=".module_common.o"' -D__KBUILD_MODNAME=kmod_.module_common.o
#include "linux/kconfig.h"
#include "linux/compiler_types.h"
#include "asm/orc_header.h"
#include "linux/build-salt.h"
#include "linux/elfnote-lto.h"
#include "linux/export-internal.h"
#include "linux/module.h"
#include "linux/errno.h"
#include "linux/types.h"
#include "linux/socket.h"
#include "linux/net.h"
#include "linux/netdevice.h"
#include "linux/icmpv6.h"
#include "net/ipv6.h"
#include "net/protocol.h"
#include "net/transp_v6.h"
#include "net/rawv6.h"
#include "net/ndisc.h"
#include "net/ip6_route.h"
#include "net/calipso.h"
#include "net/xfrm.h"
#include "linux/seg6.h"
#include "net/seg6.h"
#include "net/seg6_hmac.h"
#include "net/rpl.h"
#include "linux/ioam6.h"
#include "linux/ioam6_genl.h"
#include "net/ioam6.h"
#include "net/dst_metadata.h"

// init_net [include/linux/seq_file_net.h line 9 column 19]
struct net init_net;

// kmalloc_caches [include/linux/slab.h line 622 column 21]
kmem_buckets kmalloc_caches[NR_KMALLOC_TYPES];

// __bad_size_call_parameter [include/linux/percpu-defs.h line 310 column 13]
void __bad_size_call_parameter(void) {}

// __icmpv6_send [include/linux/icmpv6.h line 41 column 13]
void __icmpv6_send(struct sk_buff *skb, u8 type, u8 code, __u32 info,
			  const struct inet6_skb_parm *parm) {}

// __kcsan_check_access [include/linux/kcsan-checks.h line 37 column 6]
void __kcsan_check_access(const volatile void *ptr, size_t size, int type) {}

// __pskb_pull_tail [include/linux/skbuff.h line 2793 column 7]
void *__pskb_pull_tail(struct sk_buff *skb, int delta) {
	return __kmalloc(sizeof(char), GFP_KERNEL);
}

// __this_cpu_preempt_check [include/linux/percpu-defs.h line 313 column 13]
void __this_cpu_preempt_check(const char *op) {}

// __warn_printk [include/asm-generic/bug.h line 93 column 28]
void __warn_printk(const char *fmt, ...) {}

// csum_partial [arch/x86/include/asm/checksum_64.h line 129 column 15]
extern __wsum csum_partial(const void *buff, int len, __wsum sum) {
	return sum;
}

// debug_lockdep_rcu_enabled [include/linux/rcupdate.h line 350 column 5]
int debug_lockdep_rcu_enabled(void) {
	return 0;
}

// dst_discard_out [include/net/dst.h line 383 column 5]
int dst_discard_out(struct net *net, struct sock *sk, struct sk_buff *skb) {
	return 0;
}

// icmpv6_param_prob_reason [include/linux/icmpv6.h line 82 column 16]
void				icmpv6_param_prob_reason(struct sk_buff *skb,
								 u8 code, int pos,
								 enum skb_drop_reason reason) {}

// inet6_add_protocol [include/net/protocol.h line 111 column 5]
int inet6_add_protocol(const struct inet6_protocol *prot, unsigned char num) {
	return 0;
}

// inet6_del_protocol [include/net/protocol.h line 112 column 5]
int inet6_del_protocol(const struct inet6_protocol *prot, unsigned char num) {
	return 0;
}

// ip6_route_input [include/net/ip6_route.h line 80 column 6]
void ip6_route_input(struct sk_buff *skb) {}

// ipv6_chk_home_addr [include/net/addrconf.h line 122 column 5]
int ipv6_chk_home_addr(struct net *net, const struct in6_addr *addr) {
	return 0;
}

// ipv6_chk_rpl_srh_loop [include/net/addrconf.h line 125 column 5]
int ipv6_chk_rpl_srh_loop(struct net *net, const struct in6_addr *segs,
			  unsigned char nsegs) {
    return 0;
}

// ipv6_rpl_srh_compress [include/net/rpl.h line 30 column 6]
void ipv6_rpl_srh_compress(struct ipv6_rpl_sr_hdr *outhdr,
			   const struct ipv6_rpl_sr_hdr *inhdr,
			   const struct in6_addr *daddr, unsigned char n) {}

// ipv6_rpl_srh_decompress [include/net/rpl.h line 26 column 6]
void ipv6_rpl_srh_decompress(struct ipv6_rpl_sr_hdr *outhdr,
			     const struct ipv6_rpl_sr_hdr *inhdr,
			     const struct in6_addr *daddr, unsigned char n) {}

// lockdep_rcu_suspicious [include/linux/lockdep.h line 656 column 6]
void lockdep_rcu_suspicious(const char *file, const int line, const char *s) {}

// lockdep_rtnl_is_held [include/linux/rtnetlink.h line 59 column 13]
bool lockdep_rtnl_is_held(void) {
	return 0;
}

// netif_rx [include/linux/netdevice.h line 4110 column 5]
int netif_rx(struct sk_buff *skb) {
	return 0;
}

// pskb_expand_head [include/linux/skbuff.h line 1398 column 5]
int pskb_expand_head(struct sk_buff *skb, int nhead, int ntail, gfp_t gfp_mask) {
	return 0;
}

// rcu_read_lock_bh_held [include/linux/rcupdate.h line 352 column 5]
int rcu_read_lock_bh_held(void) {
	return 0;
}

// rcu_read_lock_held [include/linux/rcupdate.h line 351 column 5]
int rcu_read_lock_held(void) {
	return 0;
}

// seg6_hmac_validate_skb [include/net/seg6_hmac.h line 51 column 13]
bool seg6_hmac_validate_skb(struct sk_buff *skb) {
	return 1;
}

// seg6_push_hmac [include/net/seg6_hmac.h line 49 column 12]
int seg6_push_hmac(struct net *net, struct in6_addr *saddr,
			  struct ipv6_sr_hdr *srh) {
    return 1;
}

// sk_skb_reason_drop [include/linux/skbuff.h line 1265 column 20]
void __fix_address sk_skb_reason_drop(struct sock *sk, struct sk_buff *skb,
				      enum skb_drop_reason reason) {}

// skb_might_realloc [include/linux/skbuff.h line 2694 column 6]
void skb_might_realloc(struct sk_buff *skb) {}

// skb_pull [include/linux/skbuff.h line 2769 column 7]
void *skb_pull(struct sk_buff *skb, unsigned int len) {
	return __kmalloc(sizeof(char) * len, GFP_KERNEL);
}

// skb_push [include/linux/skbuff.h line 2759 column 7]
void *skb_push(struct sk_buff *skb, unsigned int len) {
	return __kmalloc(sizeof(char) * len, GFP_KERNEL);
}

// skb_scrub_packet [include/linux/skbuff.h line 4172 column 6]
void skb_scrub_packet(struct sk_buff *skb, bool xnet) {}

// sock_kmalloc [include/net/sock.h line 1844 column 7]
void *sock_kmalloc(struct sock *sk, int size, gfp_t priority) {
	return __kmalloc(size, GFP_KERNEL);
}

// xfrm6_input_addr [include/net/xfrm.h line 1816 column 5]
int xfrm6_input_addr(struct sk_buff *skb, xfrm_address_t *daddr,
		     xfrm_address_t *saddr, u8 proto) {
    return 0;
}
