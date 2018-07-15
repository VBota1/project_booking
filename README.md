# project_booking
Tracking time on projects/tasks

A ToDo is a list of Tasks.  
A Task is defeind by:
```
    Name
    Duration Spent on Task
    Labels (to help group tasks by various criteria)
```  

**Version: 020017000**

**Implemented:**
```
    Event Logging
    Loading/Saving ToDo from/to a file
    Adding a new Task
    Clock In a Task
    Clock Out of a Task
    CLI interface
    Report by Task
    Report by Label
    Daily activity report
    Add Time to Task
    Remove Task
    Help
    License
    Application Mode
```

**Usage Examples**
```
    ./project_booking_cli new task510 Project1 Project2
    ./project_booking_cli clockIn task510
    ./project_booking_cli report
    ./project_booking_cli clockOut task510
    ./project_booking_cli reportByLabel
    ./project_booking_cli reportForMonth 5
    ./project_booking_cli addTime task510 01:01
    ./project_booking_cli addTime task510 01:01 31.05.2021
    ./project_booking_cli delete task510
    ./project_booking_cli help
    ./project_booking_cli license
    ./project_booking_cli applicationMode
    exit
```
**Known issues:**
```
    timesamp for log messages is invalid
    days reported with command reportForMonth are not sorted 
```
  
**To be Done:**
```
    implment Gui (options: QT or https://github.com/SergioBenitez/Rocket or https://github.com/Boscop/web-view or)
    import tasks from Jira
``` 
  
