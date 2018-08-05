#-------------------------------------------------
#
# Project created by QtCreator 2018-07-08T23:14:21
#
#-------------------------------------------------

QT       += testlib

QT       -= gui

TARGET = tst_projectbookingqtui
CONFIG   += console
CONFIG   -= app_bundle

TEMPLATE = app

INCLUDEPATH += $$PWD/../../ProjectBookingQtUI/ProjectBookingQtUI

HEADERS += \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/monthreport.h \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/result.h \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/err.h \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/ok.h \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/backendhandler.h \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/standardreport.h \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/reportformats.h \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/jsonparser.h \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/projectlabelreport.h

SOURCES += tst_projectbookingqtui.cpp \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/monthreport.cpp \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/backendhandler.cpp \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/standardreport.cpp \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/jsonparser.cpp \
    ../../ProjectBookingQtUI/ProjectBookingQtUI/projectlabelreport.cpp

DEFINES += SRCDIR=\\\"$$PWD/\\\"
