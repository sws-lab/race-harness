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
#include "linux/namei.h"
#include "fs/smb/client/cifsglob.h"
#include "fs/smb/client/cifsproto.h"
#include "fs/smb/client/cifs_debug.h"
#include "fs/smb/client/smb2proto.h"
#include "fs/smb/client/cached_dir.h"

// __cpu_online_mask [include/linux/cpumask.h line 116 column 23]
struct cpumask __cpu_online_mask;

// cfid_put_wq [fs/smb/client/cifsglob.h line 2107 column 33]
struct workqueue_struct *cfid_put_wq;

// cifsFYI [fs/smb/client/cifs_debug.h line 28 column 12]
int cifsFYI;

// cifs_tcp_ses_lock [fs/smb/client/cifsglob.h line 2049 column 20]
spinlock_t		cifs_tcp_ses_lock;

// dir_cache_timeout [fs/smb/client/cifsglob.h line 2092 column 21]
unsigned int dir_cache_timeout;

// jiffies [include/linux/jiffies.h line 86 column 76]
unsigned long volatile __cacheline_aligned_in_smp __jiffy_arch_data jiffies;

// kmalloc_caches [include/linux/slab.h line 622 column 21]
kmem_buckets kmalloc_caches[NR_KMALLOC_TYPES];

// nr_cpu_ids [include/linux/cpumask.h line 30 column 21]
unsigned int nr_cpu_ids;

// serverclose_wq [fs/smb/client/cifsglob.h line 2106 column 33]
struct workqueue_struct *serverclose_wq;

// system_freezable_power_efficient_wq [include/linux/workqueue.h line 464 column 33]
struct workqueue_struct *system_freezable_power_efficient_wq;

// system_freezable_wq [include/linux/workqueue.h line 462 column 33]
struct workqueue_struct *system_freezable_wq;

// system_highpri_wq [include/linux/workqueue.h line 459 column 33]
struct workqueue_struct *system_highpri_wq;

// system_long_wq [include/linux/workqueue.h line 460 column 33]
struct workqueue_struct *system_long_wq;

// system_power_efficient_wq [include/linux/workqueue.h line 463 column 33]
struct workqueue_struct *system_power_efficient_wq;

// system_unbound_wq [include/linux/workqueue.h line 461 column 33]
struct workqueue_struct *system_unbound_wq;

// system_wq [include/linux/workqueue.h line 458 column 33]
struct workqueue_struct *system_wq;

// SMB2_close [fs/smb/client/smb2proto.h line 186 column 12]
int SMB2_close(const unsigned int xid, struct cifs_tcon *tcon,
		      u64 persistent_file_id, u64 volatile_file_id) {
	return 0;
}

// SMB2_open_free [fs/smb/client/smb2proto.h line 166 column 13]
void SMB2_open_free(struct smb_rqst *rqst) {}

// SMB2_open_init [fs/smb/client/smb2proto.h line 161 column 12]
int SMB2_open_init(struct cifs_tcon *tcon,
			  struct TCP_Server_Info *server,
			  struct smb_rqst *rqst,
			  __u8 *oplock, struct cifs_open_parms *oparms,
			  __le16 *path) {
    return 0;
}

// SMB2_query_info_free [fs/smb/client/smb2proto.h line 213 column 13]
void SMB2_query_info_free(struct smb_rqst *rqst) {}

// SMB2_query_info_init [fs/smb/client/smb2proto.h line 206 column 12]
int SMB2_query_info_init(struct cifs_tcon *tcon,
				struct TCP_Server_Info *server,
				struct smb_rqst *rqst,
				u64 persistent_fid, u64 volatile_fid,
				u8 info_class, u8 info_type,
				u32 additional_info, size_t output_len,
				size_t input_len, void *input) {
	return 0;
}

// ____wrong_branch_error [include/linux/jump_label.h line 419 column 13]
bool ____wrong_branch_error(void) {
	return 0;
}

// ___ratelimit [include/linux/ratelimit_types.h line 44 column 12]
int ___ratelimit(struct ratelimit_state *rs, const char *func) {
	return 0;
}

// __bad_size_call_parameter [include/linux/percpu-defs.h line 310 column 13]
void __bad_size_call_parameter(void) {}

// __dynamic_pr_debug [include/linux/dynamic_debug.h line 141 column 6]
void __dynamic_pr_debug(struct _ddebug *descriptor, const char *fmt, ...) {}

// __flush_workqueue [include/linux/workqueue.h line 596 column 13]
void __flush_workqueue(struct workqueue_struct *wq) {}

// __init_work [include/linux/workqueue.h line 260 column 13]
void __init_work(struct work_struct *work, int onstack) {}

// __kcsan_check_access [include/linux/kcsan-checks.h line 37 column 6]
void __kcsan_check_access(const volatile void *ptr, size_t size, int type) {}

// __list_add_valid_or_report [include/linux/list.h line 53 column 35]
bool __list_valid_slowpath __list_add_valid_or_report(struct list_head *new,
							     struct list_head *prev,
							     struct list_head *next) {
    return 0;
}

// __list_del_entry_valid_or_report [include/linux/list.h line 96 column 35]
bool __list_valid_slowpath __list_del_entry_valid_or_report(struct list_head *entry) {
	return 0;
}

// __warn_flushing_systemwide_wq [include/linux/workqueue.h line 759 column 13]
void __warn_flushing_systemwide_wq(void) {}

// __warn_printk [include/asm-generic/bug.h line 93 column 28]
void __warn_printk(const char *fmt, ...) {}

// __xadd_wrong_size [arch/x86/include/asm/cmpxchg.h line 17 column 13]
void __xadd_wrong_size(void) {}

// backup_cred [fs/smb/client/cifsproto.h line 149 column 13]
bool backup_cred(struct cifs_sb_info *) {
	return 0;
}

// cancel_delayed_work_sync [include/linux/workqueue.h line 609 column 13]
bool cancel_delayed_work_sync(struct delayed_work *dwork) {
	return 0;
}

// cancel_work_sync [include/linux/workqueue.h line 605 column 13]
bool cancel_work_sync(struct work_struct *work) {
	return 0;
}

// cifs_convert_path_to_utf16 [fs/smb/client/smb2proto.h line 29 column 16]
__le16 *cifs_convert_path_to_utf16(const char *from,
					  struct cifs_sb_info *cifs_sb) {
    return __kmalloc(sizeof(__le16), GFP_KERNEL);
}

// cifs_pick_channel [fs/smb/client/cifsproto.h line 103 column 32]
struct TCP_Server_Info *cifs_pick_channel(struct cifs_ses *ses) {
    return __kmalloc(sizeof(struct TCP_Server_Info), GFP_KERNEL);
}

// compound_send_recv [fs/smb/client/cifsproto.h line 108 column 12]
int compound_send_recv(const unsigned int xid, struct cifs_ses *ses,
			      struct TCP_Server_Info *server,
			      const int flags, const int num_rqst,
			      struct smb_rqst *rqst, int *resp_buf_type,
			      struct kvec *resp_iov) {
    return 0;
}

// delayed_work_timer_fn [include/linux/workqueue_types.h line 14 column 6]
void delayed_work_timer_fn(struct timer_list *t) {}

// dput [include/linux/dcache.h line 399 column 13]
void dput(struct dentry *) {}

// free_rsp_buf [fs/smb/client/cifsproto.h line 31 column 13]
void free_rsp_buf(int, void *) {}

// init_timer_key [include/linux/timer.h line 70 column 6]
void init_timer_key(struct timer_list *timer,
		    void (*func)(struct timer_list *), unsigned int flags,
		    const char *name, struct lock_class_key *key) {}

// lockdep_init_map_type [include/linux/lockdep.h line 128 column 13]
void lockdep_init_map_type(struct lockdep_map *lock, const char *name,
	struct lock_class_key *key, int subclass, u8 inner, u8 outer, u8 lock_type) {}

// lockref_get [include/linux/lockref.h line 49 column 6]
void lockref_get(struct lockref *lockref) {}

// lookup_positive_unlocked [include/linux/namei.h line 74 column 23]
struct dentry *lookup_positive_unlocked(const char *, struct dentry *, int) {
    return __kmalloc(sizeof(struct dentry), GFP_KERNEL);
}

// queue_delayed_work_on [include/linux/workqueue.h line 590 column 13]
bool queue_delayed_work_on(int cpu, struct workqueue_struct *wq,
			struct delayed_work *work, unsigned long delay) {
    return 0;
}

// queue_work_on [include/linux/workqueue.h line 586 column 13]
bool queue_work_on(int cpu, struct workqueue_struct *wq,
			struct work_struct *work) {
    return 0;
}

// rb_first [include/linux/rbtree.h line 46 column 24]
struct rb_node *rb_first(const struct rb_root *) {
    return __kmalloc(sizeof(struct rb_node), GFP_KERNEL);
}

// rb_next [include/linux/rbtree.h line 44 column 24]
struct rb_node *rb_next(const struct rb_node *) {
    return __kmalloc(sizeof(struct rb_node), GFP_KERNEL);
}

// rcu_is_watching [include/linux/rcutree.h line 105 column 6]
bool rcu_is_watching(void) {
	return 0;
}

// refcount_warn_saturate [include/linux/refcount.h line 116 column 6]
void refcount_warn_saturate(refcount_t *r, enum refcount_saturation_type t) {}

// smb2_parse_contexts [fs/smb/client/smb2proto.h line 284 column 5]
int smb2_parse_contexts(struct TCP_Server_Info *server,
			struct kvec *rsp_iov,
			__u16 *epoch,
			char *lease_key, __u8 *oplock,
			struct smb2_file_all_info *buf,
			struct create_posix_rsp *posix) {
	return 0;
}

// smb2_set_next_command [fs/smb/client/smb2proto.h line 132 column 13]
void smb2_set_next_command(struct cifs_tcon *tcon,
				  struct smb_rqst *rqst) {}

// smb2_set_related [fs/smb/client/smb2proto.h line 134 column 13]
void smb2_set_related(struct smb_rqst *rqst) {}

// smb2_set_replay [fs/smb/client/smb2proto.h line 135 column 13]
void smb2_set_replay(struct TCP_Server_Info *server,
			    struct smb_rqst *rqst) {}

// smb2_should_replay [fs/smb/client/smb2proto.h line 137 column 13]
bool smb2_should_replay(struct cifs_tcon *tcon,
			  int *pretries,
			  int *pcur_sleep) {}

// smb2_validate_and_copy_iov [fs/smb/client/smb2proto.h line 294 column 12]
int smb2_validate_and_copy_iov(unsigned int offset,
				      unsigned int buffer_length,
				      struct kvec *iov,
				      unsigned int minbufsize, char *data) {}

// smb3_encryption_required [fs/smb/client/smb2proto.h line 291 column 12]
int smb3_encryption_required(const struct cifs_tcon *tcon) {}
