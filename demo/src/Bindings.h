/* generated by rust_qt_binding_generator */
#ifndef BINDINGS_H
#define BINDINGS_H

#include <QObject>
#include <QAbstractItemModel>

class Person : public QObject
{
    Q_OBJECT
public:
    class Private;
private:
    Private * const d;
    Q_PROPERTY(QString userName READ userName WRITE setUserName NOTIFY userNameChanged FINAL)
    Q_PROPERTY(int age READ age NOTIFY ageChanged FINAL)
    Q_PROPERTY(bool active READ active WRITE setActive NOTIFY activeChanged FINAL)
    Q_PROPERTY(QByteArray icon READ icon WRITE setIcon NOTIFY iconChanged FINAL)
public:
    explicit Person(QObject *parent = nullptr);
    ~Person();
    QString userName() const;
    void setUserName(const QString& v);
    int age() const;
    bool active() const;
    void setActive(bool v);
    QByteArray icon() const;
    void setIcon(const QByteArray& v);
signals:
    void userNameChanged();
    void ageChanged();
    void activeChanged();
    void iconChanged();
private:
    QString m_userName;
    int m_age;
    bool m_active;
    QByteArray m_icon;
};

class Directory : public QAbstractItemModel
{
    Q_OBJECT
public:
    class Private;
private:
    Private * const d;
    Q_PROPERTY(QString path READ path WRITE setPath NOTIFY pathChanged FINAL)
public:
    explicit Directory(QObject *parent = nullptr);
    ~Directory();
    QString path() const;
    void setPath(const QString& v);

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    QHash<int, QByteArray> roleNames() const override;
signals:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
signals:
    void pathChanged();
private:
    QString m_path;
};

class TestTree : public QAbstractItemModel
{
    Q_OBJECT
public:
    class Private;
private:
    Private * const d;
    Q_PROPERTY(QString path READ path WRITE setPath NOTIFY pathChanged FINAL)
public:
    explicit TestTree(QObject *parent = nullptr);
    ~TestTree();
    QString path() const;
    void setPath(const QString& v);

    int columnCount(const QModelIndex &parent = QModelIndex()) const override;
    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QModelIndex index(int row, int column, const QModelIndex &parent = QModelIndex()) const override;
    QModelIndex parent(const QModelIndex &index) const override;
    bool hasChildren(const QModelIndex &parent = QModelIndex()) const override;
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;
    bool canFetchMore(const QModelIndex &parent) const override;
    void fetchMore(const QModelIndex &parent) override;
    QHash<int, QByteArray> roleNames() const override;
signals:
    // new data is ready to be made available to the model with fetchMore()
    void newDataReady(const QModelIndex &parent) const;
signals:
    void pathChanged();
private:
    QString m_path;
};
#endif // BINDINGS_H
