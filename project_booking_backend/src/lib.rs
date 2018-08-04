extern crate todo;
extern crate formaters;
extern crate logger;
extern crate chrono;
extern crate report;

use todo::*;
use logger::*;
use formaters::dmy_format;
use std::slice::Iter;
use std::time::Duration;
use chrono::{NaiveDate, Datelike, Local};
use report::*;

extern crate serde;
extern crate serde_json;

//TODO MEDIUM PRIO GUI (QT)
//TODO LOW PRIO import tasks from Jira
//TODO LOW PRIO export tasks to PTT

pub const NEW: &'static str = "new";
pub const CLOCKIN: &'static str = "clockIn";
pub const CLOCKOUT: &'static str = "clockOut";
pub const REPORT: &'static str = "report";
pub const REPORTBYLABEL: &'static str = "reportByLabel";
pub const REPORTFORMONTH: &'static str = "reportForMonth";
pub const ADDTIME: &'static str = "addTime";
pub const REMOVETIME: &'static str = "removeTime";
pub const DELETE: &'static str = "delete";
pub const HELP: &'static str = "help";
pub const LICENSE: &'static str = "license";
pub const APPLICATIONMODE: &'static str = "applicationMode";
pub const EXIT: &'static str = "exit";

pub fn help() -> String {
    format!("Project booking Version 023018010 MIT License Copyright (c) 2018 V Bota
The source code is available at: https://github.com/VBota1/project_booking
Service mode usage: ./project_booking_cli command [task][labels][time]
Application mode usage: command [task][labels][time]
  Supported commands:
\t{0: <18} Creates a new task. Can be followed by multiple labels.
\t{1: <18} Starts to monitor time for the indicated task. Must be followed by a task name.
\t{2: <18} Stops to monitor time for the indicated task and adds the duration between this event and clockIn event to the task. Must be followed by a task name.
\t{3: <18} Prints out a report of all the recorded tasks.
\t{4: <18} Prints out a report of all the duration spend on each label.
\t{11: <18} Prints out a report of the time spent on each task of each day of the specified month. Must be followed by a month argument: example 05 for May.
\t{5: <18} Add the specified time to the specified task. Must be followed by a task name. Must be followed by the time to add in the format hh:mm . Can be followed by the date for which to add the time. The date must be in the format dd.mm.yyyy .
\t{12: <18} Remove the specified time of the specified task. Must be followed by a task name. Must be followed by the time to remove in the format hh:mm . Can be followed by the date for which to remove the time. The date must be in the format dd.mm.yyyy .
\t{6: <18} Removes the specified task from the recordings. Must be followed by a task name.
\t{7: <18} Prints out this help text.
\t{8: <18} Prints out License information.
\t{9: <18} Enters application mode. Changes made in application mode are saved to nonvolatile storage upon calling command {10}.
\t{10: <18} Exits application mode after saving all changes made to nonvolatile storage. Will return an error if called outside application mode.
  Examples:
\t./project_booking_cli {0} task510 Project1 Project2
\t./project_booking_cli {1} task510
\t./project_booking_cli {3}
\t./project_booking_cli {2} task510
\t./project_booking_cli {4}
\t./project_booking_cli {11} 5
\t./project_booking_cli {5} task510 01:01
\t./project_booking_cli {5} task510 01:01 31.05.2021
\t./project_booking_cli {12} task510 01:01
\t./project_booking_cli {12} task510 01:01 31.05.2021
\t./project_booking_cli {6} task510
\t./project_booking_cli {7}
\t./project_booking_cli {8}
\t./project_booking_cli {9}
\t{10}
    ", NEW, CLOCKIN, CLOCKOUT, REPORT, REPORTBYLABEL, ADDTIME, DELETE, HELP, LICENSE, APPLICATIONMODE, EXIT, REPORTFORMONTH, REMOVETIME)
}

pub fn delete(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    let task_name = match args.nth(0) {
        Some(task) => { task.clone() },
        None => {
            return Err(warn(format!("No task name received. \"{}\"", recommend_help())));
        },
    };

    to_do.remove_task(task_name).log_result()
}

pub fn report(to_do: &ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    match serde_json::to_string(&to_do.to_complete_report()) {
        Ok(data_as_string) => {
            Ok(data_as_string)
        },
        Err(_) => {
            Err(error(format!("Data could not be serialized")))
        }
    }
}

pub fn report_time_on_labels(to_do: &ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    match serde_json::to_string(&to_do.to_time_on_labels_report()) {
        Ok(data_as_string) => {
            Ok(data_as_string)
        },
        Err(_) => {
            Err(error(format!("Data could not be serialized")))
        }
    }
}

pub fn daily_activity_report(mut args: Iter<String>, to_do: &ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    let month = match args.nth(0) {
        Some(value) => {
            match value.parse::<u32>() {
                Ok(value) => { value },
                Err(_) => {
                    return Err(warn(format!("Month argument \"{}\" could not be parsed as u32", value)));
                }
            }
        },
        None => {
            return Err(warn(format!("Month argument is missing. {}", recommend_help())));
        }
    };

    match serde_json::to_string(&to_do.to_month_jurnal_report(month)) {
        Ok(data_as_string) => {
            Ok(data_as_string)
        },
        Err(_) => {
            Err(error(format!("Data could not be serialized")))
        }
    }
}

fn no_tasks_recorderd_message() -> String {
    format!("No tasks are recorded")
}

pub fn clock_out(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    match args.nth(0) {
        Some(task_name) => {
            to_do.clock_out(task_name.to_string()).log_result()
        },
        None => {
            Err(warn(format!("Please supply a valid task name for the clock out operation. {}", recommend_help())))
        },
    }
}

pub fn clock_in(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    match args.nth(0) {
        Some(task_name) => {
            to_do.clock_in(task_name.to_string()).log_result()
        },
        None => {
            Err(warn(format!("Please supply a valid task name for the clock in operation. {}", recommend_help())))
        },
    }
}

pub fn create_new_task_from_arguments(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    match args.nth(0) {
        Some(task) => {
            let labels: Vec<String> = args.cloned().collect();
            to_do.add(task.clone(), labels).log_result()
        },
        None => {
            Err(warn(format!("No task name received. \"{}\"", recommend_help())))
        },
    }
}

pub fn add_time(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    let task_name = match args.nth(0) {
        Some(task) => { task },
        None => {
            return Err(warn(format!("No task name received. \"{}\"", recommend_help())));
        },
    };

    let (hours, minutes) = parse_as_time(args.nth(0))?;

    let date: Option<NaiveDate> = parse_as_date(args.nth(0))?;

    let secs = (hours * 3600 + minutes * 60) as u64;

    to_do.add_time_spent_to_task(task_name.clone(), date, Duration::new(secs, 0))
}

fn parse_as_time(to_be_parsed: Option<&String>) -> Result<(u32, u32), String> {
    match to_be_parsed {
        Some(time_argument) => {
            let time: Vec<String> = time_argument.split(':').map(|s| format!("{}", s)).collect();
            let error_message = format!("Time to be added to the task, \"{}\" ,is not in the expected format. \"{}\"", time_argument, recommend_help());

            let mut h = match time.get(0) {
                Some(h) => { h },
                None => { return Err(warn(error_message)); }
            };
            let mut min = match time.get(1) {
                Some(min) => { min },
                None => { return Err(warn(error_message)); }
            };

            let h = match h.parse::<u32>() {
                Ok(value) => { value },
                Err(_) => { return Err(warn(error_message)); }
            };

            let min = match min.parse::<u32>() {
                Ok(value) => { value },
                Err(_) => { return Err(warn(error_message)); }
            };

            Ok((h, min))
        },
        None => {
            return Err(warn(format!("No time to be added to the task was received. \"{}\"", recommend_help())));
        }
    }
}

fn parse_as_date(to_be_parsed: Option<&String>) -> Result<Option<NaiveDate>, String> {
    match to_be_parsed {
        Some(date_argument) => {
            match NaiveDate::parse_from_str(date_argument, dmy_format().as_str()) {
                Ok(result) => { Ok(Some(result)) },
                Err(_) => {
                    return Err(warn(format!("Date for which to add time, \"{}\" ,is not in the expected format. \"{}\"", date_argument, recommend_help())));
                }
            }
        }
        None => {
            warn(format!("No date for which to add time was received. Current date will be used"));
            Ok(None)
        }
    }
}

pub fn remove_time(mut args: Iter<String>, to_do: &mut ToDo) -> Result<String, String> {
    if 0 == to_do.count() { return Err(warn(no_tasks_recorderd_message())); }

    let task_name = match args.nth(0) {
        Some(task) => { task },
        None => {
            return Err(warn(format!("No task name received. \"{}\"", recommend_help())));
        },
    };

    let (hours, minutes) = parse_as_time(args.nth(0))?;

    let date: Option<NaiveDate> = parse_as_date(args.nth(0))?;

    let secs = (hours * 3600 + minutes * 60) as u64;

    to_do.remove_time_spent_to_task(task_name.clone(), date, Duration::new(secs, 0))
}

pub fn store(to_do: ToDo) -> Result<String, String> {
    forced_store(to_do)
}

pub fn forced_store(to_do: ToDo) -> Result<String, String> {
    to_do.save(None).log_result()
}

pub fn get_to_do(load_file: Option<String>) -> ToDo {
    match load(load_file) {
        Ok(todo) => {
            trace(format!("ToDo loaded from database"));
            todo
        },
        Err(error) => {
            warn(error.to_string());
            ToDo::new()
        },
    }
}

pub fn recommend_help() -> String {
    format!("Call \"project_booking help\" for additional information.")
}

pub fn license() -> String {
    format!("
The complete source code is stored at: https://github.com/VBota1/project_booking

MIT License
Copyright (c) 2018 V Bota
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


3\'rd party LICENSES:

BSD 3 Clause LICENSE for extern crate simple_logging:
Copyright 2017 Isabela Schulze
Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:
1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

MIT LICENSE for extern crates serde, serde_json and serde_derive:
Copyright (c) 2014 The Rust Project Developers
Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the \"Software\"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:
The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

MIT LICENSE for extern crate chrono
The MIT License (MIT)
Copyright (c) 2014, Kang Seonghoon.
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.


I used unmodified QT 5.11 core libraries from the Qt website: https://www.qt.io/download.
To replace these libraries simply download the source code from: https://github.com/VBota1/project_booking
and recompile it using your version of the library,
GNU LESSER GENERAL PUBLIC LICENSE for the QT libraries used
                       Version 3, 29 June 2007
 Copyright (C) 2007 Free Software Foundation, Inc. <http://fsf.org/>
 Everyone is permitted to copy and distribute verbatim copies
 of this license document, but changing it is not allowed.
  This version of the GNU Lesser General Public License incorporates
the terms and conditions of version 3 of the GNU General Public
License, supplemented by the additional permissions listed below.
  0. Additional Definitions.
  As used herein, \"this License\" refers to version 3 of the GNU Lesser
General Public License, and the \"GNU GPL\" refers to version 3 of the GNU
General Public License.
  \"The Library\" refers to a covered work governed by this License,
other than an Application or a Combined Work as defined below.
  An \"Application\" is any work that makes use of an interface provided
by the Library, but which is not otherwise based on the Library.
Defining a subclass of a class defined by the Library is deemed a mode
of using an interface provided by the Library.
  A \"Combined Work\" is a work produced by combining or linking an
Application with the Library.  The particular version of the Library
with which the Combined Work was made is also called the \"Linked
    Version\".
  The \"Minimal Corresponding Source\" for a Combined Work means the
Corresponding Source for the Combined Work, excluding any source code
for portions of the Combined Work that, considered in isolation, are
based on the Application, and not on the Linked Version.
  The \"Corresponding Application Code\" for a Combined Work means the
object code and/or source code for the Application, including any data
and utility programs needed for reproducing the Combined Work from the
Application, but excluding the System Libraries of the Combined Work.
  1. Exception to Section 3 of the GNU GPL.
  You may convey a covered work under sections 3 and 4 of this License
without being bound by section 3 of the GNU GPL.
  2. Conveying Modified Versions.
  If you modify a copy of the Library, and, in your modifications, a
facility refers to a function or data to be supplied by an Application
that uses the facility (other than as an argument passed when the
facility is invoked), then you may convey a copy of the modified
version:
   a) under this License, provided that you make a good faith effort to
   ensure that, in the event an Application does not supply the
   function or data, the facility still operates, and performs
   whatever part of its purpose remains meaningful, or
   b) under the GNU GPL, with none of the additional permissions of
   this License applicable to that copy.
  3. Object Code Incorporating Material from Library Header Files.
  The object code form of an Application may incorporate material from
a header file that is part of the Library.  You may convey such object
code under terms of your choice, provided that, if the incorporated
material is not limited to numerical parameters, data structure
layouts and accessors, or small macros, inline functions and templates
(ten or fewer lines in length), you do both of the following:
   a) Give prominent notice with each copy of the object code that the
   Library is used in it and that the Library and its use are
   covered by this License.
   b) Accompany the object code with a copy of the GNU GPL and this license
   document.
  4. Combined Works.
  You may convey a Combined Work under terms of your choice that,
taken together, effectively do not restrict modification of the
portions of the Library contained in the Combined Work and reverse
engineering for debugging such modifications, if you also do each of
the following:
   a) Give prominent notice with each copy of the Combined Work that
   the Library is used in it and that the Library and its use are
   covered by this License.
   b) Accompany the Combined Work with a copy of the GNU GPL and this license
   document.
   c) For a Combined Work that displays copyright notices during
   execution, include the copyright notice for the Library among
   these notices, as well as a reference directing the user to the
   copies of the GNU GPL and this license document.
   d) Do one of the following:
       0) Convey the Minimal Corresponding Source under the terms of this
       License, and the Corresponding Application Code in a form
       suitable for, and under terms that permit, the user to
       recombine or relink the Application with a modified version of
       the Linked Version to produce a modified Combined Work, in the
       manner specified by section 6 of the GNU GPL for conveying
       Corresponding Source.
       1) Use a suitable shared library mechanism for linking with the
       Library.  A suitable mechanism is one that (a) uses at run time
       a copy of the Library already present on the user's computer
       system, and (b) will operate properly with a modified version
       of the Library that is interface-compatible with the Linked
       Version.
   e) Provide Installation Information, but only if you would otherwise
   be required to provide such information under section 6 of the
   GNU GPL, and only to the extent that such information is
   necessary to install and execute a modified version of the
   Combined Work produced by recombining or relinking the
   Application with a modified version of the Linked Version. (If
   you use option 4d0, the Installation Information must accompany
   the Minimal Corresponding Source and Corresponding Application
   Code. If you use option 4d1, you must provide the Installation
   Information in the manner specified by section 6 of the GNU GPL
   for conveying Corresponding Source.)
  5. Combined Libraries.
  You may place library facilities that are a work based on the
Library side by side in a single library together with other library
facilities that are not Applications and are not covered by this
License, and convey such a combined library under terms of your
choice, if you do both of the following:
   a) Accompany the combined library with a copy of the same work based
   on the Library, uncombined with any other library facilities,
   conveyed under the terms of this License.
   b) Give prominent notice with the combined library that part of it
   is a work based on the Library, and explaining where to find the
   accompanying uncombined form of the same work.
  6. Revised Versions of the GNU Lesser General Public License.
  The Free Software Foundation may publish revised and/or new versions
of the GNU Lesser General Public License from time to time. Such new
versions will be similar in spirit to the present version, but may
differ in detail to address new problems or concerns.
  Each version is given a distinguishing version number. If the
Library as you received it specifies that a certain numbered version
of the GNU Lesser General Public License \"or any later version\"
applies to it, you have the option of following the terms and
conditions either of that published version or of any later version
published by the Free Software Foundation. If the Library as you
received it does not specify a version number of the GNU Lesser
General Public License, you may choose any version of the GNU Lesser
General Public License ever published by the Free Software Foundation.
  If the Library as you received it specifies that a proxy can decide
whether future versions of the GNU Lesser General Public License shall
apply, that proxy's public statement of acceptance of any version is
permanent authorization for you to choose that version for the
Library.
    ")
}

#[cfg(test)]
mod test_set;
