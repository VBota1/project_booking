# project_booking
Tracking time on projects/tasks

A ToDo is a list of Tasks.  
A Task is defeind by:
```
    Name
    Duration Spent on Task
    Labels (to help group tasks by various criteria)
```  

**Version: 023018010**

**Implemented:**
```
    Adding a new Task
    Remove Task
    Clock In a Task
    Clock Out of a Task
    Add Time to Task
    Remove Time from Task
    Report by Task / Standard Report
    Daily activity report
    Report by Label / Project    
    Help
    License
    Application Mode
        
    Event Logging
    Loading/Saving ToDo from/to a file
    CLI interface
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
    ./project_booking_cli removeTime task510 01:01
    ./project_booking_cli removeTime task510 01:01 31.05.2021    
    ./project_booking_cli delete task510
    ./project_booking_cli help
    ./project_booking_cli license
    ./project_booking_cli applicationMode
    exit
```
**Known issues:**
```
    timesamp for log messages is invalid
```
  
**To be Done:**
```
    implment Gui (options: QT or https://github.com/SergioBenitez/Rocket or https://github.com/Boscop/web-view or)
    import tasks from Jira
``` 
  
