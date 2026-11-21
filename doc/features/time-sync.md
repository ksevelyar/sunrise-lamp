# Time Sync

## Requirements
* The lamp synchronizes its clock with an NTP server at startup.
* The lamp uses the configured timezone to determine local schedule time, including daylight saving time changes.
* A valid RTC time can drive the schedule while synchronization is pending.
* Without a valid time, the lamp stays off until synchronization succeeds.
* The lamp retries failed synchronization attempts.

## User scenarios

### 1. Lamp synchronizes time on startup
1. The lamp starts without a valid RTC time and stays off.
2. The lamp connects to Wi-Fi and obtains an IP address.
3. The lamp resolves the NTP server and receives the current time.
4. The lamp sets its clock and RTC to the received time.
5. The lamp follows the schedule at the current local time.

### 2. Lamp uses valid RTC time while synchronization is pending
1. The lamp starts with a valid RTC time.
2. Time synchronization has not yet succeeded.
3. The lamp follows the schedule at the RTC's local time.
4. When synchronization succeeds, the lamp updates its clock and RTC to the received time.

### 3. Lamp retries failed time synchronization
1. The lamp starts without a valid RTC time.
2. A time synchronization attempt fails.
3. The lamp remains off and retries synchronization.
4. A later attempt succeeds and the lamp follows the schedule at the current local time.

### 4. Lamp follows local time across a daylight saving change
1. The configured timezone is `Europe/Berlin`.
2. On 29 March 2026, the synchronized time advances from 00:59:59 UTC to 01:00:00 UTC.
3. The schedule time advances from 01:59:59 to 03:00:00 local time.

## Implementation
* The firmware seeds its clock from a valid RTC time and synchronizes with `pool.ntp.org` after network configuration.
* Failed NTP attempts are retried. The firmware stores a successful result in the RTC.
