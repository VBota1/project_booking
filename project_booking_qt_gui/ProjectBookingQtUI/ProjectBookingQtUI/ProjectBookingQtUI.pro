#-------------------------------------------------
#
# Project created by QtCreator 2018-07-08T23:41:23
#
#-------------------------------------------------

QT       += core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

TARGET = ProjectBookingQtUI
TEMPLATE = app


SOURCES += main.cpp\
        projectbooking.cpp \
    monthreport.cpp \
    backendhandler.cpp \
    standardreport.cpp \
    jsonparser.cpp \
    projectlabelreport.cpp \
    statusdisplay.cpp \
    reports.cpp \
    addtimeform.cpp \
    taskmenu.cpp \
    newtaskform.cpp \
    helpform.cpp \
    licenseform.cpp

HEADERS  += projectbooking.h \
    err.h \
    monthreport.h \
    ok.h \
    result.h \
    backendhandler.h \
    standardreport.h \
    reportformats.h \
    jsonparser.h \
    projectlabelreport.h \
    statusdisplay.h \
    reports.h \
    addtimeform.h \
    taskmenu.h \
    newtaskform.h \
    helpform.h \
    licenseform.h

FORMS    += projectbooking.ui
