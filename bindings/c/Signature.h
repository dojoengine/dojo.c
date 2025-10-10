#ifndef Signature_H
#define Signature_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "FieldElement.d.h"

#include "Signature.d.h"






Signature* Signature_new(const FieldElement* r, const FieldElement* s);

void Signature_r(const Signature* self, DiplomatWrite* write);

void Signature_s(const Signature* self, DiplomatWrite* write);

void Signature_destroy(Signature* self);





#endif // Signature_H
