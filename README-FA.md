# 🔒 DoH-Tester

🇺🇸 [English](README.md) | 🇷🇺 [Русский](README-RU.md) | 🇨🇳 [中文](README-ZH.md) | 🇮🇷 [فارسی](README-FA.md)

[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)
<div dir="rtl">
ابزاری پرسرعت و چندنخی برای تست نقاط انتهایی DNS-over-HTTPS (DoH) با تشخیص هوشمند پروتکل، فیلترگذاری قابل تنظیم و مدیریت خودکار لیست‌ها.

## 📋 معرفی

 ابزار DoH-tester نقاط انتهایی DoH را در مقیاس بالا اعتبارسنجی می‌کند و اتصال TCP، هندشیک TLS و حل واقعی DNS را در چندین پروتکل (فرمت وایر GET/POST و API مبتنی بر JSON) بررسی می‌کند. این ابزار برای مدیران شبکه، فعالان حریم خصوصی و توسعه‌دهندگانی طراحی شده که نیاز به حفظ دسترسی پایدار و بدون سانسور به DNS در محیط‌های شبکه‌ای خصمانه دارند.

---

### ویژگی‌های کلیدی

- ✅ تست اتصال TCP (IPv4 و IPv6)
- 🔐 بررسی هندشیک TLS (با حالت ناامن اختیاری)
- 🌐 حل DNS از طریق:
  - ‏DoH GET (فرمت وایر)
  - ‏DoH POST (فرمت وایر)
  - ‏DoH GET (API مبتنی بر JSON)
- ⚡ تست موازی با استفاده از استخر نخ‌ها
- 🧠 دسته‌بندی هوشمند (WORKING / FLAKY / BLOCKED)
- 📊 اندازه‌گیری تأخیر (میلی‌ثانیه)
- 🧾 خروجی جدولی خوانا برای انسان
- 🧹 حالت خروجی تمیز (فقط URLها)
- 📦 خروجی JSON اختیاری (با برچسب زمانی خودکار یا نام دلخواه)
- 📁 پیکربندی از طریق فایل `config.json`
- 🗂 پاک‌سازی خودکار اختیاری نقاط انتهایی سالم از فایل منبع

---

## 🎯 موارد استفاده

| سناریو                           | نقش DoH-Tester                                                                                                   |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| **دور زدن سانسور**               | کشف سریع رزولورهای DoH که واقعاً کار می‌کنند برای عبور از مسدودسازی مبتنی بر DNS و دسترسی به پلتفرم‌های فیلترشده |
| **نگه‌داری ابزارهای حریم خصوصی** | ساخت و به‌روزرسانی لیست‌های قابل‌اعتماد DoH برای VPNها، پروکسی‌ها، تونل‌ها یا تنظیمات مرورگر                     |
| **بهینه‌سازی عملکرد**            | اندازه‌گیری تأخیر برای یافتن سریع‌ترین رزولور متناسب با موقعیت شما                                               |
| **ممیزی شبکه**                   | اعتبارسنجی زیرساخت DoH در شبکه‌های سازمانی یا ISP                                                                |
| **پایش زیرساخت**                 | بررسی سلامت خودکار سرورهای خصوصی DoH                                                                             |

<details>
<summary><b>🔓 دور زدن سانسور (برای باز شدن کلیک کنید)</b></summary>

اتصال به پلتفرم‌های فیلترشده مانند YouTube، Instagram، Twitter/X و سایت‌های خبری با حل دامنه‌ها از طریق اتصال HTTPS رمزنگاری‌شده و عبور از فیلترینگ مبتنی بر DNS و ربایش[gfw resist HTTPS proxy](https://github.com/GFW-knocker/gfw_resist_HTTPS_proxy) DNS

**نحوه عملکرد:**

* ‏‏درخواست‌های استاندارد DNS (UDP پورت ۵۳) رمزنگاری نشده و به‌راحتی توسط فایروال‌ها رهگیری می‌شوند
* ‏‏DoH درخواست‌های DNS را داخل ترافیک HTTPS (پورت ۴۴۳) کپسوله می‌کند و از مرور وب معمولی قابل تشخیص نیست
* ‏‏مفید در شبکه‌های به‌شدت سانسورشده که در آن‌ها:
‏
  * ‏DNS استاندارد مسموم شده است (برگرداندن IPهای نادرست)
  * ‏نام دامنه‌ها در سطح رزولور DNS مسدود شده‌اند
  * ‏فیلترینگ SNI اعمال شده ولی رمزنگاری DNS هنوز مسدود نشده است

**نکته مهم:** برای مثال اگر VPNها یا IPهای Cloudflare در **لایه IP** مسدود شده باشند (فایروال بسته‌ها به آن IPها را Drop کند)، DoH به‌تنهایی نمی‌تواند دسترسی به آن IPهای خاص را بازگرداند. اما DoH می‌تواند به شما کمک کند:

1. نقاط انتهایی جایگزین و فعال که هنوز مسدود نشده‌اند را پیدا کنید
2. دامنه‌های VPN را به IP حل کنید (در صورتی که فقط DNS مسدود شده باشد، نه خود IPها)
3. به نقاط انتهایی جایگزین یا «domain-fronted» در CDNهایی که در سطح IP مسدود نیستند دسترسی پیدا کنید

</details>

<details>
<summary><b>🔐 نگه‌داری ابزارهای حریم خصوصی (برای باز شدن کلیک کنید)</b></summary>

حفظ دسترسی به زیرساخت حریم خصوصی در شرایطی که روش‌های استاندارد کشف کار نمی‌کنند:
<div dir="rtl" align="right">

* ‏**دسترسی به دامنه‌های مسدودشده VPN:** اگر دامنه ارائه‌دهنده VPN شما (مثلاً `vpn-provider.com`) از طریق ربایش DNS مسدود شده باشد ولی سرورها در سطح IP مسدود نشده باشند، می‌توانید با استفاده از نقاط انتهایی DoH سالم، آدرس واقعی سرورها را حل کرده و اتصال را حفظ کنید.
* ‏**تونل‌سازی DNS:** استفاده از نقاط انتهایی تأییدشده DoH به‌عنوان لایه انتقال برای ابزارهای تونل‌سازی DNS مانند:‏
  * ‏[dnstt](https://www.bamsoftware.com/software/dnstt/) – تونل TCP روی DNS که از طریق رزولورهای DoH کار می‌کند
  * ‏[DNSCrypt-proxy](https://github.com/DNSCrypt/dnscrypt-proxy) – امکان مسیریابی از طریق DoH با رله‌های ناشناس
  * ‏[Iodine](https://github.com/yarrick/iodine) – تونل IP روی DNS (نیازمند UDP، ولی قابل استفاده از DoH برای bootstrap)
* **ابزارهای دور زدن سانسور در مرحله Bootstrap:** بسیاری از ابزارهای ضدسانسور (Tor bridgeها، Shadowsocks، WireGuard) ابتدا نیاز به حل یک سرور bootstrap دارند. اگر این جستجوی اولیه DNS مسموم شده باشد، ابزار قادر به اتصال نیست. حل اولیه از طریق DoH، IPهای صحیح را برای راه‌اندازی ابزارها فراهم می‌کند.
</div>
</details>

<details>
<summary><b>⚡ بهینه‌سازی عملکرد (برای باز شدن کلیک کنید)</b></summary>

یافتن رزولور بهینه متناسب با شرایط شبکه شما:

* اندازه‌گیری هم‌زمان تأخیر به چندین نقطه انتهایی DoH
* شناسایی بهینه‌سازی‌های مسیریابی جغرافیایی (برخی ISPها به PoPهای نزدیک‌تر مسیریابی می‌کنند)
* مقایسه سرعت حل بین پیاده‌سازی‌های فرمت وایر و API مبتنی بر JSON
* ساخت لیست‌های رزولور آگاه از موقعیت که سریع‌ترین گزینه را به‌طور خودکار انتخاب می‌کنند

</details>

<details>
<summary><b>🏢 ممیزی شبکه (برای باز شدن کلیک کنید)</b></summary>

اعتبارسنجی دسترس‌پذیری و انطباق زیرساخت DoH:

* تست این‌که کدام رزولورهای عمومی DoH از شبکه‌های سازمانی قابل دسترسی هستند
* بررسی پاسخ‌دهی صحیح سرورهای خصوصی/داخلی DoH
* شناسایی رهگیری TLS (میان‌افزارهایی که اتصال DoH را مختل می‌کنند)
* تولید گزارش‌های انطباق که قابلیت حریم خصوصی DNS را در بخش‌های مختلف شبکه نشان می‌دهد

</details>

## ✨ فهرست کامل قابلیت‌ها

<details>
<summary><strong>🔍 قابلیت‌های اصلی تست</strong></summary>

* **پشتیبانی از پروتکل DoH**: استاندارد RFC 8484 (فرمت وایر DNS) از طریق GET و POST، به‌همراه API مبتنی بر JSON (سازگار با Google و Cloudflare)
* **اعتبارسنجی لایه‌ای**: اتصال TCP → هندشیک TLS → حل DNS در سطح برنامه DoH
* **تشخیص هوشمند پروتکل**: تست خودکار فرمت وایر و API JSON در صورت پشتیبانی
* **شبکه دوپشته**: پشتیبانی از IPv4 و IPv6 با بازگشت خودکار
* **تست امن برای ISP**: انجام حل واقعی DNS بدون تحریک مسموم‌سازی یا فیلترینگ DNS

</details>

<details>
<summary><strong>⚡ عملکرد و پایداری</strong></summary>

* **موتور تست موازی**: استخر نخ قابل تنظیم برای تست سریع نقاط انتهایی
* **منطق تلاش مجدد مقاوم**: چندین تلاش برای هر نقطه انتهایی با آستانه موفقیت قابل تنظیم
* **اندازه‌گیری تأخیر**: زمان‌سنجی دقیق هر کوئری در میلی‌ثانیه
* **خروج امن با Ctrl+C**: خاموشی نرم با ذخیره نتایج جزئی
* **دسته‌بندی هوشمند**: طبقه‌بندی نقاط انتهایی به **WORKING**، **FLAKY** یا **BLOCKED**

</details>

<details>
<summary><strong>🔐 امنیت و عیب‌یابی</strong></summary>

* **اعتبارسنجی TLS**: بررسی گواهی و هندشیک با حالت ناامن اختیاری
* **بازرسی گواهی**: ثبت موضوع گواهی TLS و جزئیات Cipher
* **طبقه‌بندی خطا**: تفکیک مسدودسازی TCP، رهگیری TLS و خطاهای سطح برنامه DoH
* **گزارش IP حل‌شده**: نمایش نتایج واقعی حل DNS برای تأیید

</details>

<details>
<summary><strong>📊 خروجی و گزارش‌دهی</strong></summary>

* **فرمت‌های خروجی انعطاف‌پذیر**:
‏
  * جدول‌های خوانا برای انسان
  * لیست‌های تمیز فقط شامل URL (مناسب اسکریپت)
  * ‏JSON قابل‌خواندن توسط ماشین
* **خروجی دارای زمان**: برچسب زمانی خودکار ISO 8601 (یا نام فایل دلخواه)
* **مرتب‌سازی نتایج**: خروجی JSON مرتب‌شده بر اساس تأخیر (سریع‌ترین در ابتدا)
* **فیلتر فقط سالم‌ها**: امکان نمایش یا خروجی گرفتن فقط از نقاط انتهایی سالم

</details>

<details>
<summary><strong>🗂 مدیریت لیست و فایل</strong></summary>

* **حالت پاک‌سازی خودکار**: حذف نقاط انتهایی سالم از لیست منبع
* **حفاظت با پشتیبان**: ایجاد فایل‌های `.backup` قبل از اعمال تغییرات
* **حفظ کامنت‌ها**: نگه‌داشتن توضیحات و قالب‌بندی در لیست‌ها
* **لیست‌های خودترمیم**: کمک به نگه‌داری مجموعه‌های به‌روز و قابل‌اعتماد DoH

</details>

<details>
<summary><strong>🧠 پیکربندی و کاربری</strong></summary>

* **کاملاً قابل پیکربندی**: کنترل همه پیش‌فرض‌ها از طریق `config.json`
* **تنظیم تایم‌اوت و محدودیت‌ها**: کنترل دقیق تلاش‌ها، تعداد نخ‌ها و آستانه‌ها
* **حالت خروجی تمیز**: خروجی حداقلی برای اتوماسیون و پایپ‌لاین‌های شِل

</details>

## 🚀 نصب

### پیش‌نیازها

* [Python](http://python.org/downloads/) **نسخه 3.8 به بالا** (توصیه می‌شود)

### کلون یا دانلود

```bash
git clone https://github.com/SkipTutorial/doh_tester.git
cd doh_tester
```

همچنین میتوانید فایل مخصوص ویندوز را از [اینجا](https://github.com/SkipTutorial/doh_tester/releases) دانلود کنید

### نصب وابستگی‌ها

```bash
pip install requests dnspython
```

# بررسی نصب

```bash
python test_doh.py --help
```

## نحوه استفاده و دستورات

### استفاده پایه

```bash
python test_doh.py <domain> [options]
```
### آرگومان‌های خط فرمان

| آرگومان             | پیش‌فرض       | توضیح                                         |
| ------------------- | ------------- | --------------------------------------------- |
| `domain`            | (الزامی)      | دامنه برای حل (مثلاً `example.com`)           |
| `config--`          | `config.json` | مسیر فایل پیکربندی                            |
| `doh-file--`        | `doh.txt`     | مسیر فایل حاوی URLهای DoH                     |
| `timeout--`         | `8.0`         | تایم‌اوت هر عملیات (ثانیه)                    |
| `workers--`         | `20`          | تعداد نخ‌های موازی                            |
| `attempts--`        | `3`           | تعداد تلاش‌های DNS برای هر نقطه انتهایی       |
| `min-success--`     | `2`           | حداقل پاسخ موفق برای وضعیت WORKING            |
| `insecure--`        | `False`       | رد کردن بررسی گواهی TLS                       |
| `output--`          | (دارای زمان)  | مسیر فایل خروجی                               |
| `working-only--`    | `False`       | نمایش فقط نتایج WORKING                       |
| `no-working-only--` | -             | نمایش همه نتایج (نادیده‌گرفتن تنظیم پیکربندی) |
| `clean-output--`    | `False`       | خروجی فقط URLهای سالم (هر خط یک URL)          |
| `json-output--`     | `False`       | نوشتن خروجی JSON (خودکار یا مسیر مشخص)        |

### مثال‌های استفاده

#### تست ساده

```bash
python test_doh.py example.com
```

#### نمایش فقط نقاط انتهایی سالم

```bash
python test_doh.py example.com --working-only
```

#### خروجی تمیز (فقط لیست URL)

```bash
python test_doh.py example.com --clean-output
```

#### پیکربندی سفارشی

```bash
python test_doh.py example.com \
  --doh-file my_doh_list.txt \
  --timeout 10 \
  --workers 30 \
  --attempts 5 \
  --min-success 3
```

#### خروجی JSON

```bash
# نام فایل JSON با برچسب زمانی خودکار
python test_doh.py example.com --json-output

# نام فایل JSON مشخص
python test_doh.py example.com --json-output results.json
```

#### تست DoH خصوصی (گواهی‌های Self-Signed)

```bash
python test_doh.py internal.domain --insecure
```

#### ترکیب چند گزینه

```bash
python test_doh.py example.com --working-only --clean-output --json-output --output results.txt
```

---

## فایل پیکربندی

این ابزار از یک فایل پیکربندی JSON (به‌صورت پیش‌فرض `config.json`) برای کنترل همه تنظیمات استفاده می‌کند.

### پیکربندی پیش‌فرض

```json
{
  "doh_file": "doh.txt",
  "output_file": "",
  "timeout": 8.0,
  "workers": 20,
  "attempts": 3,
  "min_success": 2,
  "remove_working_from_doh_file": false,
  "working_only": false,
  "json_output": false,
  "show_headers": true,
  "show_status": true,
  "show_doh_url": true,
  "show_host": true,
  "show_doh_ip": true,
  "show_target_ip": false,
  "show_ping": true
}
```

### گزینه‌های پیکربندی

#### تنظیمات فایل و خروجی

| گزینه         | نوع    | توضیح                               |
| ------------- | ------ | ----------------------------------- |
| `doh_file`    | string | مسیر فایل حاوی URLهای DoH           |
| `output_file` | string | فایل خروجی پیش‌فرض (خالی = با زمان) |

#### پارامترهای تست

| گزینه         | نوع     | توضیح                           |
| ------------- | ------- | ------------------------------- |
| `timeout`     | float   | تایم‌اوت هر عملیات (ثانیه)      |
| `workers`     | integer | تعداد نخ‌های موازی              |
| `attempts`    | integer | تعداد تلاش برای هر نقطه انتهایی |
| `min_success` | integer | حداقل موفقیت برای وضعیت WORKING |

#### مدیریت فایل

| گزینه                          | نوع     | توضیح                           |
| ------------------------------ | ------- | ------------------------------- |
| `remove_working_from_doh_file` | boolean | حذف موارد WORKING از فایل منبع  |
| `working_only`                 | boolean | نمایش پیش‌فرض فقط نتایج WORKING |

#### تنظیمات خروجی JSON

| گزینه         | نوع            | توضیح                                                      |
| ------------- | -------------- | ---------------------------------------------------------- |
| `json_output` | boolean/string | `false` (بدون JSON)، `true`/`"auto"` (با زمان) یا نام فایل |

#### تنظیمات نمایش

| گزینه            | نوع     | توضیح                |
| ---------------- | ------- | -------------------- |
| `show_headers`   | boolean | نمایش عنوان ستون‌ها  |
| `show_status`    | boolean | نمایش ستون STATUS    |
| `show_doh_url`   | boolean | نمایش ستون URL       |
| `show_host`      | boolean | نمایش ستون HOST      |
| `show_doh_ip`    | boolean | نمایش ستون DOH_IP    |
| `show_target_ip` | boolean | نمایش ستون TARGET_IP |
| `show_ping`      | boolean | نمایش ستون PING_MS   |

### فرمت فایل DoH

فایل URLهای DoH (به‌صورت پیش‌فرض `doh.txt`) از کامنت و خطوط خالی پشتیبانی می‌کند:

```text
# سرورهای عمومی DoH
https://cloudflare-dns.com/dns-query
https://dns.google/dns-query
https://dns.quad9.net/dns-query

# خصوصی / داخلی
https://doh.internal.company/dns-query
```

---

## قالب ترمینال

![terminal](terminal.PNG)

## فرمت‌های خروجی

### خروجی متنی استاندارد
<div dir="ltr">

```
# Generated: 2026-02-4 08:50:45
STATUS    URL                                   HOST                DOH_IP           PING_MS
------------------------------------------------------------------------------------------------
WORKING   https://cloudflare-dns.com/dns-query  cloudflare-dns.com  104.16.249.249   45.2
WORKING   https://dns.google/dns-query          dns.google          8.8.8.8          32.1
BLOCKED   https://blocked.doh.server/dns-query  blocked.server      -                -
FLAKY     https://unreliable.doh/dns-query      unreliable.doh      192.0.2.1        120.5
```
<div/>

### خروجی تمیز

در صورت استفاده از `clean-output--`:

```
https://cloudflare-dns.com/dns-query
https://dns.google/dns-query
```

### خروجی JSON

```json
[
  {
    "status": "WORKING",
    "url": "https://cloudflare-dns.com/dns-query",
    "host": "cloudflare-dns.com",
    "port": 443,
    "tcp_ok": true,
    "tls_ok": true,
    "tls_info": "cipher=('TLS_AES_256_GCM_SHA384', 'TLSv1.3', 256)...",
    "successes": 3,
    "attempts": 3,
    "target_ips": "93.184.216.34",
    "doh_server_ip": "104.16.249.249",
    "method": "GET-wire",
    "latency_ms": "45.2",
    "notes": ""
  }
]
```

### دسته‌بندی وضعیت‌ها
<div dir="rtl" align="right">

| وضعیت       | معنی                                                                                       |
| ----------- | ------------------------------------------------------------------------------------------ |
| **WORKING** | نقطه انتهایی تمام تست‌ها را گذرانده است (TCP، TLS و تعداد کوئری‌های موفق بزرگتر یا مساوی min_success)    |
| **FLAKY**   | نقطه انتهایی به‌صورت جزئی کار می‌کند (برخی کوئری‌ها موفق بوده‌اند ولی کمتر از min_success) |
| **BLOCKED** | نقطه انتهایی ناموفق است (خطای TCP/TLS یا بدون کوئری موفق)                                  |

---

</div>

## نکات و بهترین روش‌ها

### تنظیم عملکرد

#### تنظیم تعداد نخ‌ها بر اساس اتصال شما:

   * ‏کند/ناپایدار: `workers 5--`
   * ‏سریع/پایدار: `workers 50--`

#### افزایش تایم‌اوت برای شبکه‌های کند:

   ```bash
   python test_doh.py example.com --timeout 15
   ```

#### کاهش تلاش‌ها برای بررسی سریع:

   ```bash
   python test_doh.py example.com --attempts 1 --min-success 1
   ```

### تست پایداری

#### استفاده از تلاش‌های بیشتر برای اعتبارسنجی محیط تولید:

   ```bash
   python test_doh.py example.com --attempts 5 --min-success 4
   ```

#### تست چند دامنه:

   ```bash
   for domain in example.com google.com cloudflare.com; do
     python test_doh.py $domain --json-output --output ${domain}.txt
   done
   ```

### اتوماسیون

#### کران‌جاب برای پایش:

   ```bash
   # اجرا روزانه ساعت ۳ صبح
   0 3 * * * cd /path/to/test_doh && python test_doh.py monitor.domain --json-output >> cron.log 2>&1
   ```

#### اسکریپت تولید لیست تمیز:

   ```bash
   #!/bin/bash
   python test_doh.py example.com --clean-output --working-only --output working_doh.txt --json-output doh_results.json
   ```

### عیب‌یابی

1. **همه نقاط انتهایی BLOCKED هستند**: بررسی کنید آیا DoH توسط ISP یا فایروال مسدود شده است
2. **خطاهای TLS**: برای گواهی‌های Self-Signed از `insecure--` استفاده کنید
3. **Timeout**: مقدار `timeout--` را افزایش دهید یا `workers--` را کاهش دهید
4. **عدم وجود نتیجه**: بررسی کنید فایل `doh.txt` شامل URLهای معتبر باشد

---

## ملاحظات امنیتی

* گزینه `insecure--` بررسی گواهی TLS را غیرفعال می‌کند؛ فقط برای تست سرورهای خصوصی استفاده شود.
* کوئری‌های DoH رمزنگاری شده‌اند، اما سرور مقصد می‌تواند درخواست‌های DNS شما را مشاهده کند.
* برای افزونگی، تست را روی چند ارائه‌دهنده DoH مختلف انجام دهید.

---

## مجوز

مجوز MIT — جزئیات در فایل LICENSE.

---

## مشارکت

لطفاً Issue یا Pull Request ارسال کنید.

---

## قدردانی‌ها
<div dir="rtl" align="right">

* ‏[RFC 8484](https://tools.ietf.org/html/rfc8484) – کوئری‌های DNS روی HTTPS (DoH)
* ‏[dnspython](https://www.dnspython.org/) – ابزار DNS برای پایتون
* ‏[requests](https://requests.readthedocs.io/) – کتابخانه HTTP برای پایتون
</div>
</div>
