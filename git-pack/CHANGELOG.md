### 0.9.0 (2021-08-??)

- **renames / moves / visibility**
   - `find::Find`  and `find::FindExt` only in `Find` and `FindExt` (not in `find` anymore)
   - `data::output::count::Count` -> `data::output::Count`
   - `data::output::entry::Entry` -> `data::output::Entry`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find_existing_*` -> `Find::find_*`
   - `Find::find()-> `Find::try_find()`
   - `bundle::Bundle` -> `Bundle`
   - `bundle::Error` -> `bundle::init::Error`
   - `pub tree::` -> `pub(crate) cache::delta::`
 
* **new methods**
   - `Find::find_tag_iter()`
