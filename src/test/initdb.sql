CREATE DATABASE `userdb`
use `userdb`;
-- userdb.book definition

CREATE TABLE `book`
(
    `id`         bigint NOT NULL,
    `book_name`  varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL,
    `key`        varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL,
    `author`     varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL,
    `content`    text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,
    `created_at` datetime                                                      DEFAULT NULL,
    `updated_at` datetime                                                      DEFAULT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
-- userdb.dept definition

CREATE TABLE `dept`
(
    `DEPTNO` int                                                          NOT NULL COMMENT '部门编号',
    `DNAME`  varchar(14) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '部门名称',
    `LOC`    varchar(13) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '部门地址',
    PRIMARY KEY (`DEPTNO`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='部门表';
-- userdb.emp definition

CREATE TABLE `emp`
(
    `empno`    int                                                           NOT NULL COMMENT '雇员编号',
    `ename`    varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '雇员姓名',
    `job`      varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '雇员职位',
    `mgr`      int                                                           DEFAULT NULL COMMENT '雇员上级领导编号',
    `hiredate` date                                                          NOT NULL COMMENT '入职时间',
    `sal` double DEFAULT NULL COMMENT '基本工资',
    `comm` double DEFAULT NULL COMMENT '奖金',
    `deptno`   int                                                           NOT NULL COMMENT '部门编号',
    PRIMARY KEY (`empno`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='雇员表';
-- userdb.salgrade definition

CREATE TABLE `salgrade`
(
    `grade` int DEFAULT NULL COMMENT '工资等级',
    `losal` int DEFAULT NULL COMMENT '此等级最低工资',
    `hisal` int DEFAULT NULL COMMENT '此等级最高工资'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='工资等级表';
-- userdb.t_sys_role definition

CREATE TABLE `t_sys_role`
(
    `id`        int         NOT NULL AUTO_INCREMENT,
    `role_code` varchar(20) NOT NULL,
    `role_name` varchar(50) NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
-- userdb.t_sys_user definition

CREATE TABLE `t_sys_user`
(
    `id`                      int         NOT NULL AUTO_INCREMENT,
    `username`                varchar(20) NOT NULL,
    `password`                varchar(60) NOT NULL,
    `enabled`                 bit(1)   DEFAULT b'1',
    `account_non_expired`     bit(1)   DEFAULT b'1',
    `credentials_non_expired` bit(1)   DEFAULT b'1',
    `account_non_locked`      bit(1)   DEFAULT b'1',
    `create_datetime_at`      datetime DEFAULT NULL,
    `update_datetime_at`      datetime DEFAULT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
-- userdb.t_sys_user_role definition

CREATE TABLE `t_sys_user_role`
(
    `id`      int NOT NULL AUTO_INCREMENT,
    `user_id` int NOT NULL,
    `role_id` int NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
-- userdb.t_user definition

CREATE TABLE `t_user`
(
    `id`       bigint       NOT NULL AUTO_INCREMENT,
    `name`     varchar(100) NOT NULL,
    `age`      smallint     DEFAULT NULL,
    `gender`   varchar(50)  DEFAULT NULL,
    `province` varchar(50)  DEFAULT NULL,
    `city`     varchar(50)  DEFAULT NULL,
    `address`  varchar(150) DEFAULT NULL,
    `phone`    varchar(20)  DEFAULT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=15 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

INSERT INTO userdb.dept (DEPTNO, DNAME, LOC)
VALUES (10, 'ACCOUNTING', 'NEW YORK'),
       (20, 'RESEARCH', 'DALLAS'),
       (30, 'SALES', 'CHICAGO'),
       (40, 'OPERATIONS', 'BOSTON');

INSERT INTO userdb.emp (empno, ename, job, mgr, hiredate, sal, comm, deptno)
VALUES (7369, 'SMITH', 'CLERK', 7902, '1980-12-17', 800.0, NULL, 20),
       (7499, 'ALLEN', 'SALESMAN', 7698, '1981-02-20', 1600.0, 300.0, 30),
       (7521, 'WARD', 'SALESMAN', 7698, '1981-02-22', 1250.0, 500.0, 30),
       (7566, 'JONES', 'MANAGER', 7839, '1981-04-02', 2975.0, NULL, 20),
       (7654, 'MARTIN', 'SALESMAN', 7698, '1981-09-28', 1250.0, 1400.0, 30),
       (7698, 'BLAKE', 'MANAGER', 7839, '1981-05-01', 2850.0, NULL, 30),
       (7782, 'CLARK', 'MANAGER', 7839, '1981-06-09', 2450.0, NULL, 10),
       (7788, 'SCOTT', 'CLERK', 7566, '1987-04-19', 3000.0, NULL, 20),
       (7839, 'KING', 'PRESIDENT', NULL, '1981-11-17', 5000.0, NULL, 10),
       (7844, 'TURNER', 'SALESMAN', 7698, '1981-09-08', 1500.0, 0.0, 30),
       (7876, 'ADAMS', 'CLERK', 7788, '1987-05-23', 1100.0, NULL, 20),
       (7900, 'JAMES', 'CLERK', 7698, '1981-12-03', 950.0, NULL, 30),
       (7902, 'FORD', 'ANALYST', 7566, '1981-12-03', 3000.0, NULL, 20),
       (7934, 'MILLER', 'CLERK', 7782, '1982-01-23', 1300.0, NULL, 10);

INSERT INTO userdb.salgrade (grade, losal, hisal)
VALUES (1, 700, 1200),
       (2, 1201, 1400),
       (3, 1401, 2000),
       (4, 2001, 3000),
       (5, 3001, 9999);

INSERT INTO userdb.t_sys_role (role_code, role_name)
VALUES ('ROLE_admin', '超级管理员'),
       ('ROLE_user', '普通用户'),
       ('ROLE_audit', '审批用户');

INSERT INTO userdb.t_sys_user (username, password, enabled, account_non_expired, credentials_non_expired,
                               account_non_locked, create_datetime_at, update_datetime_at)
VALUES ('bob', '$2a$10$Y3tIGA6t4HGUKEArDgKZPOqV6dQ8UKgYEZPHZ0j7StsFEY85INvw2', 1, 1, 1, 1, '2023-06-08 23:53:21', NULL),
       ('admin', '$2a$10$rsWgh6bBZan0q/cepa0dqeuHY3hqUhNr9OKMwNQ4g1Ec4SolLKDGK', 1, 1, 1, 1, '2023-07-08 23:53:28',
        NULL),
       ('gannon', '$2a$10$2MI0tL6ZZdttWehkIZi0COXqi1g7i16uIXr9EHCP8csupyXOKTmY2', 1, 1, 1, 1, '2023-05-08 23:53:31',
        NULL);

INSERT INTO userdb.t_sys_user_role (user_id, role_id)
VALUES (1, 2),
       (2, 1),
       (3, 3);

INSERT INTO userdb.t_user (name, age, gender, province, city, address, phone)
VALUES ('test', 18, '1', '四川', '成都', '天府街', '13311011101'),
       ('gannon', 22, '', 'sichuan', 'chengdu', '', '');