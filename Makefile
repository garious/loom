all:

##############################
OBJS+=out/buf.o
out/buf.o:src/buf.c

CEXES+=cov/buf_t
cov/buf_t:src/buf_t.c src/buf.c 

COVS+=cov/buf.c.cov
cov/buf.c.cov:cov/buf_t

##############################
OBJS+=out/toaster.o
out/toaster.o:src/toaster.c

##############################
OBJS+=out/config.o
out/config.o:src/config.c

CEXES+=cov/config_t
cov/config_t:src/config_t.c src/config.c out/toaster.o

COVS+=cov/config.c.cov
cov/config.c.cov:cov/config_t

##############################
#rules
all:$(OBJS) $(COVS)

clean:
	rm -rf out cov *.gcno *.gcda *.gcov

CFLAGS+=-Iinc -fPIC -g -Wall -Werror -O3 -std=c99 -fvisibility=hidden -ffunction-sections

DEP_FLAGS=-MMD -MP -MF $(@:%=%.d)
LD_FLAGS=-Wl,--gc-sections

OBJ_DEPS=$(OBJS:%=%.d)
-include $(OBJ_DEPS)

$(OBJS):
	@mkdir -p $(@D)
	$(CC) -o $@ -c $(filter %.c, $^) $(CFLAGS) $(DEP_FLAGS)

EXE_DEPS=$(EXES:%=%.d)
-include $(EXE_DEPS)

$(EXES):
	@mkdir -p $(@D)
	$(CC) -o $@ $^ $(CFLAGS) $(DEP_FLAGS)

DLL_DEPS=$(DLLS:%=%.d)
-include $(DLL_DEPS)

$(DLLS):
	mkdir -p $(@D)
	$(CC) -o $@ $^ $(CFLAGS) -shared -ldl $(DEP_FLAGS)

export GCOV_PREFIX=cov
export GCOV_PREFIX_STRIP=$(words $(subst /, ,$(PWD)))
$(COVS):
	@mkdir -p $(@D)
	$<
	mv *.gcno $(@D)/ || echo ok
	gcov -r -c -b -o $(@D) $(notdir $(@:%.cov=%)) | tee $<.cov.out
	mv *.gcov $(@D)/ || echo ok
	@grep "Branches executed:100" $<.cov.out
	@grep "Lines executed:100" $<.cov.out
	touch $@

CEXE_DEPS=$(CEXES:%=%.d)
-include $(CEXE_DEPS)

$(CEXES):
	@mkdir -p $(@D)
	$(CC) -o $@ $(filter-out %.h, $^) $(DEP_FLAGS) $(LD_FLAGS) $(CFLAGS) -fsanitize=address -fsanitize=leak -fsanitize=undefined -coverage -ldl -DTOASTER 

$$%:;@$(call true)$(info $(call or,$$$*))
