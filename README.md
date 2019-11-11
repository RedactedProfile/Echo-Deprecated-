=== Echo ===

A Minimalist Cache Server written with Rust; inspired by but doesn't copy Redis


== Config ==

Configuration is handled through one very simple `conf.yaml` file.  Here is the config in full with their default values

```
server:
  host: 127.0.0.1
  port: 9875
  backup:
    path: ~
    interval: 7200
  log:
    path: ~
```

== Syntax ==

The syntax to Echo is very simple, deliberate, and minimal. Echo emphasises efficient minimalism. It wants to be spun up and fully autonomous with minimal maintenance.  


= Create / Update Solid Entry =  

Create a new solid entry that doesn't expire. 
*note:* this is not permanent. If Echo shuts down or restarts for any reason, even solid entries will be lost. 

```
SET mykeyname "my value"
```

= Create / Update Self Expiring Entry = 

Creates a new self expiring entry with a TTL (time to live) in seconds from it's creation.

```
SET mykeyname "my value" 3600
```

= Add a TTL to existing Entry = 

Updates a solid entry to a self expiring entry with a countdown in seconds.

```
TTL mykeyname 3600
```

= Delete Entry = 

Will remove the entry from the datastorage. 

```
DEL mykeyname 
```

= Get Entry = 

Returns a single entry's value if it exists, or null if not

```
GET mykeyname
```

= Get Multiple Entries = 

Will return an array of entries in the order provided. `null` will be supplied for each entry that doesn't exist.

```
GET mykeyname,anotherkeyname,yetanother
```

= Check Entry Existance = 

Returns a null value if the entry doesn't exist (as in, never did, was deleted, or ttl expired)

```
CHK mykeyname
```
