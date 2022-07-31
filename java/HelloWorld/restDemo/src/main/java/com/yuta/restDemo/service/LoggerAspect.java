package com.yuta.restDemo.service;

import com.yuta.restDemo.user.UserController;
import org.apache.commons.logging.Log;
import org.apache.commons.logging.LogFactory;
import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;
import org.springframework.stereotype.Component;

@Aspect
@Component
public class LoggerAspect {

    private Log log = LogFactory.getLog(UserController.class);

    //    @Around("@annotation(com.yuta.restDemo.interfaces.LogAspect)")
    @Around("execution(* com.yuta.restDemo.user.*.*(..))")
    public Object logExecutionTime(ProceedingJoinPoint joinPoint) throws Throwable {
        log.info("-----------------  ------------------");
        return joinPoint.proceed();
    }
}
