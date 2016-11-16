extern crate libc;

// Supported address families.
pub const AF_UNSPEC: libc::c_ushort = 0;
pub const AF_UNIX: libc::c_ushort = 1;      /* Unix domain sockets          */
pub const AF_LOCAL: libc::c_ushort = 1;      /* POSIX name for AF_UNIX       */
pub const AF_INET: libc::c_ushort = 2;      /* Internet IP Protocol         */
pub const AF_AX25: libc::c_ushort = 3;      /* Amateur Radio AX.25          */
pub const AF_IPX: libc::c_ushort = 4;      /* Novell IPX                   */
pub const AF_APPLETALK: libc::c_ushort = 5;      /* AppleTalk DDP                */
pub const AF_NETROM: libc::c_ushort = 6;      /* Amateur Radio NET/ROM        */
pub const AF_BRIDGE: libc::c_ushort = 7;      /* Multiprotocol bridge         */
pub const AF_ATMPVC: libc::c_ushort = 8;      /* ATM PVCs                     */
pub const AF_X25: libc::c_ushort = 9;      /* Reserved for X.25 project    */
pub const AF_INET6: libc::c_ushort = 10;      /* IP version 6                 */
pub const AF_ROSE: libc::c_ushort = 11;      /* Amateur Radio X.25 PLP       */
pub const AF_DECnet: libc::c_ushort = 12;      /* Reserved for DECnet project  */
pub const AF_NETBEUI: libc::c_ushort = 13;      /* Reserved for 802.2LLC project*/
pub const AF_SECURITY: libc::c_ushort = 14;      /* Security callback pseudo AF */
pub const AF_KEY: libc::c_ushort = 15;      /* PF_KEY key management API */
pub const AF_NETLINK: libc::c_ushort = 16;
pub const AF_ROUTE: libc::c_ushort = AF_NETLINK; /* Alias to emulate 4.4BSD */
pub const AF_PACKET: libc::c_ushort = 17;      /* Packet family                */
pub const AF_ASH: libc::c_ushort = 18;      /* Ash                          */
pub const AF_ECONET: libc::c_ushort = 19;      /* Acorn Econet                 */
pub const AF_ATMSVC: libc::c_ushort = 20;      /* ATM SVCs                     */
pub const AF_RDS: libc::c_ushort = 21;      /* RDS sockets                  */
pub const AF_SNA: libc::c_ushort = 22;      /* Linux SNA Project (nutters!) */
pub const AF_IRDA: libc::c_ushort = 23;      /* IRDA sockets                 */
pub const AF_PPPOX: libc::c_ushort = 24;      /* PPPoX sockets                */
pub const AF_WANPIPE: libc::c_ushort = 25;      /* Wanpipe API Sockets */
pub const AF_LLC: libc::c_ushort = 26;      /* Linux LLC                    */
pub const AF_IB: libc::c_ushort = 27;      /* Native InfiniBand address    */
pub const AF_MPLS: libc::c_ushort = 28;      /* MPLS */
pub const AF_CAN: libc::c_ushort = 29;      /* Controller Area Network      */
pub const AF_TIPC: libc::c_ushort = 30;      /* TIPC sockets                 */
pub const AF_BLUETOOTH: libc::c_ushort = 31;      /* Bluetooth sockets            */
pub const AF_IUCV: libc::c_ushort = 32;      /* IUCV sockets                 */
pub const AF_RXRPC: libc::c_ushort = 33;      /* RxRPC sockets                */
pub const AF_ISDN: libc::c_ushort = 34;      /* mISDN sockets                */
pub const AF_PHONET: libc::c_ushort = 35;      /* Phonet sockets               */
pub const AF_IEEE802154: libc::c_ushort = 36;      /* IEEE802154 sockets           */
pub const AF_CAIF: libc::c_ushort = 37;      /* CAIF sockets                 */
pub const AF_ALG: libc::c_ushort = 38;      /* Algorithm sockets            */
pub const AF_NFC: libc::c_ushort = 39;      /* NFC sockets                  */
pub const AF_VSOCK: libc::c_ushort = 40;      /* vSockets                     */
pub const AF_KCM: libc::c_ushort = 41;      /* Kernel Connection Multiplexor*/
pub const AF_QIPCRTR: libc::c_ushort = 42;      /* Qualcomm IPC Router          */
pub const AF_MAX: libc::c_ushort = 43;      /* For now.. */

pub mod ffi;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod linux;

pub use linux::*;
