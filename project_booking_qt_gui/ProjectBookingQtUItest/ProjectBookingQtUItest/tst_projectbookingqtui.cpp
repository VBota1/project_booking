#include <QString>
#include <QtTest>
#include "backendhandler.h"

class ProjectBookingQtUItestTest : public QObject
{
    Q_OBJECT

public:
    ProjectBookingQtUItestTest();

private Q_SLOTS:
    void deserializeMonthReport();
    void deserializeStandardReport();
    void deserializeProjectReport();
};

ProjectBookingQtUItestTest::ProjectBookingQtUItestTest()
{
}

void ProjectBookingQtUItestTest::deserializeMonthReport()
{
    BackendHandler backend;
    Result< QList<Data> > report = backend.reportForMonth(6);

    QVERIFY2(!report.hasError,"reportForMonth(6) returned: " + report.err().toLocal8Bit());

    QCOMPARE(report.ok().first().date,QString("30.06.2018"));

    QCOMPARE (report.ok().first().tasks.first().task,QString("task512"));

    QCOMPARE(report.ok().first().tasks.first().time_spent,QString("00:00:05"));

    QCOMPARE(report.ok().first().tasks.first().labels.first(),QString("project_1"));
}

void ProjectBookingQtUItestTest::deserializeStandardReport()
{
    BackendHandler backend;
    Result< QList<TaskReport> > report = backend.standardReport();

    QVERIFY2(!report.hasError,"standardReport returned: " + report.err().toLocal8Bit());

    QVERIFY2(report.ok().count()>0,"the standardReport returned has at least one entry");

    QCOMPARE(report.ok().first().name,QString("task512"));

    QCOMPARE (report.ok().first().time_spent,QString("00:00:05"));

    QVERIFY2(report.ok().first().labels.count()>0,"the task 510 has at least one label");

    QCOMPARE(report.ok().first().labels.first(),QString("project_1"));
}

void ProjectBookingQtUItestTest::deserializeProjectReport()
{

    BackendHandler backend;
    Result< QList<LabelReport> > report = backend.projectLabelReport();

    QVERIFY2(!report.hasError,"labelReport returned: " + report.err().toLocal8Bit());

    QVERIFY2(report.ok().count()>0,"the labelReport returned has at least one entry");

    QString firstlabel = report.ok().first().label;
    QVERIFY2((firstlabel==QString("project_2")||firstlabel==QString("project_1")),"The returned label is project_1 or project_2");

    QCOMPARE (report.ok().first().time_spent,QString("00:00:02"));
}

QTEST_APPLESS_MAIN(ProjectBookingQtUItestTest)

#include "tst_projectbookingqtui.moc"
